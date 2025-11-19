use std::collections::HashMap;
use std::rc::Rc;

pub type EBNFState = u64;
pub type EBNFStateKey = u16;

const DEPTH_BIT_SHIFT: usize = 48; // ツリーの深さ
const GROUP_BIT_SHIFT: usize = 32; // 同じ深さにおけるグループ番号
const PARENT_GROUP_BIT_SHIFT: usize = 16;
const CHILDREN_GROUP_BIT_SHIFT: usize = 0;
const BIT_MASK: EBNFState = 0xFFFF;
const DEPTH_BIT_MASK: EBNFState = BIT_MASK << DEPTH_BIT_SHIFT;
const CHILDREN_GROUP_BIT_MASK: EBNFState = BIT_MASK << CHILDREN_GROUP_BIT_SHIFT;
const PARENT_GROUP_BIT_MASK: EBNFState = BIT_MASK << PARENT_GROUP_BIT_SHIFT;
const GROUP_BIT_MASK: EBNFState = BIT_MASK << GROUP_BIT_SHIFT;

#[derive(Debug)]
pub(crate) struct EBNF<'a> {
    pub name: &'a str,                               // 定義したルール名
    expr: Rc<EBNFNode<'a>>,                          // ツリー構造(ルールの中身)
    state_map: HashMap<EBNFState, Rc<EBNFNode<'a>>>, // ルールの位置(状態)に応じたマップ
    full_state_map: HashMap<EBNFState, EBNFState>,   // 部分的な状態キーから完全な状態へのマップ
}

impl<'a> EBNF<'a> {
    pub fn new(name: &'a str, expr: EBNFNode<'a>) -> Self {
        let expr = Rc::new(expr);
        let state_list = make_state_pair_list(&expr);
        let mut state_map = HashMap::new();
        let mut full_state_map = HashMap::new();
        for (key, value) in state_list {
            let state_key = key & (DEPTH_BIT_MASK | GROUP_BIT_MASK);
            state_map.insert(state_key, value);
            full_state_map.insert(state_key, key);
        }

        Self {
            name,
            expr,
            state_map,
            full_state_map,
        }
    }

    pub fn get_node(&self, state: &EBNFState) -> Option<&EBNFNode<'a>> {
        self.state_map.get(state).map(|node| &**node)
    }

    pub fn root(&self) -> EBNFState {
        make_state_key(0, 0, 0, 0)
    }

    // 親ノードに移動
    pub fn parent(&self, state: EBNFState) -> Option<EBNFState> {
        let state = state & (DEPTH_BIT_MASK | GROUP_BIT_MASK);
        let &full_state = self.full_state_map.get(&state)?;
        let depth = (full_state & DEPTH_BIT_MASK) >> DEPTH_BIT_SHIFT;
        if depth == 0 {
            return None;
        }

        let parent_depth = depth - 1;
        let parent_depth_key = parent_depth << DEPTH_BIT_SHIFT;
        let parent_group_number = (full_state & PARENT_GROUP_BIT_MASK) >> PARENT_GROUP_BIT_SHIFT;
        let parent_group_key = parent_group_number << GROUP_BIT_SHIFT;

        Some(parent_depth_key | parent_group_key)
    }

    // 隣のノードへ移動
    // 同じグループの末端であれば親ノードの隣のノードへ
    // その親が末端であればさらにその親を見ていく
    pub fn next_group(&self, state: EBNFState) -> Option<EBNFState> {
        let state = state & (DEPTH_BIT_MASK | GROUP_BIT_MASK);
        let &full_state = self.full_state_map.get(&state)?;
        let depth_key = full_state & DEPTH_BIT_MASK;
        let group = (full_state & GROUP_BIT_MASK) >> GROUP_BIT_SHIFT;
        let next_group_key = (group + 1) << GROUP_BIT_SHIFT;

        // 単純に1加算した値が次のグループかを判断できないので
        // 親を参照し親のデータから次のグループに移動するかを判断する
        let Some(parent_state) = self.parent(state) else {
            // 親はいないが次のグループが存在する場合
            let next_state = depth_key | next_group_key;
            return if self.state_map.contains_key(&next_state) {
                Some(next_state)
            } else {
                None
            };
        };

        // 次のグループがない場合親ノードへ
        let &full_parent_state = self.full_state_map.get(&parent_state)?;
        let group_start_point =
            (full_parent_state & CHILDREN_GROUP_BIT_MASK) >> CHILDREN_GROUP_BIT_SHIFT;
        let parent_node = self.get_node(&parent_state)?;
        let count = get_child_count(parent_node) as u64;
        let group_limit = group_start_point + count;
        let group_position = group + 1;
        if group_limit < group_position + 1 {
            // グループが存在しない場合はさらに親を見る
            return self.next_group(parent_state);
        }

        let next_state = depth_key | next_group_key;

        Some(next_state)
    }

    // 子のノードに移動する
    // 子ノードがなければ同グループの隣のノードへ
    // グループの末端かつ子ノードがなければ親ノードへ
    pub fn step_in(&self, state: EBNFState) -> Option<(&EBNFNode<'a>, EBNFState)> {
        let state = state & (DEPTH_BIT_MASK | GROUP_BIT_MASK);
        let &full_state = self.full_state_map.get(&state)?;
        let depth = (full_state & DEPTH_BIT_MASK) >> DEPTH_BIT_SHIFT;
        let child_depth_key = (depth + 1) << DEPTH_BIT_SHIFT;
        let child_group_key = full_state & CHILDREN_GROUP_BIT_MASK;
        let child_state = child_depth_key | child_group_key;

        let Some(node) = self.state_map.get(&child_state) else {
            return self.step_over(state); // 子がいない場合はstep_overと同じ
        };

        Some((node, child_state))
    }

    // 親ノードの隣のノードへ移動
    pub fn step_out(&self, state: EBNFState) -> Option<(&EBNFNode<'a>, EBNFState)> {
        let parent_state = self.parent(state)?;
        let parent_next_state = self.next_group(parent_state)?;
        let parent_next_node = self.get_node(&parent_next_state)?;
        Some((parent_next_node, parent_next_state))
    }

    // 同じグループ内の隣のノードへ
    // 同じグループの末端であれば親ノードの隣へ
    pub fn step_over(&self, state: EBNFState) -> Option<(&EBNFNode<'a>, EBNFState)> {
        let next_state = self.next_group(state)?;
        let next_node = self.get_node(&next_state)?;
        Some((next_node, next_state))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum EBNFNode<'a> {
    Expansion(&'a str),            // Hoge
    Concat(Vec<Rc<EBNFNode<'a>>>), // Hoge Fuga
    Or(Vec<Rc<EBNFNode<'a>>>),     // Hoge | Fuga
    // Hoge? Hoge* Hoge+ Hoge{3} Hoge{7,} Hoge{2, 5}
    Repeat {
        node: Rc<EBNFNode<'a>>,
        min: u64,
        max: Option<u64>,
    },
    Group(Rc<EBNFNode<'a>>), // (Hoge)
    Literal(&'a str),        // "hogefuga"
}

#[derive(Debug)]
pub(crate) enum Quantifier {
    Question,                 // ?
    Plus,                     // +
    Star,                     // *
    Braces(u64, Option<u64>), // {
}

fn make_state_key(
    depth: EBNFStateKey,
    group: EBNFStateKey,
    parent_group: EBNFStateKey,
    child_group: EBNFStateKey,
) -> EBNFState {
    let depth = (depth as EBNFState) << DEPTH_BIT_SHIFT;
    let group = (group as EBNFState) << GROUP_BIT_SHIFT;
    let parent_group = (parent_group as EBNFState) << PARENT_GROUP_BIT_SHIFT;
    let child_group = (child_group as EBNFState) << CHILDREN_GROUP_BIT_SHIFT;
    depth | group | parent_group | child_group
}

fn make_state_pair_list<'a>(expr: &Rc<EBNFNode<'a>>) -> Vec<(EBNFState, Rc<EBNFNode<'a>>)> {
    let mut group_count_map = HashMap::new();
    let mut vec = vec![];
    let mut stack = vec![(expr.clone(), 0, 0)];
    while let Some((current_node, depth, parent_group_number)) = stack.pop() {
        let group_number = *group_count_map
            .entry(depth)
            .and_modify(|e| *e += 1)
            .or_insert(0);
        let child_group_number = group_count_map
            .get(&(depth + 1))
            .map(|e| *e + 1)
            .unwrap_or(0);

        match &*current_node {
            EBNFNode::Expansion(_) => {
                let key =
                    make_state_key(depth, group_number, parent_group_number, child_group_number);
                vec.push((key, current_node));
            }
            EBNFNode::Concat(nodes) | EBNFNode::Or(nodes) => {
                let key =
                    make_state_key(depth, group_number, parent_group_number, child_group_number);
                vec.push((key, current_node.clone()));

                for node in nodes.iter().rev() {
                    stack.push((node.clone(), depth + 1, group_number));
                }
            }
            EBNFNode::Repeat {
                node,
                min: _,
                max: _,
            } => {
                let key =
                    make_state_key(depth, group_number, parent_group_number, child_group_number);
                vec.push((key, current_node.clone()));
                stack.push((node.clone(), depth + 1, group_number));
            }
            EBNFNode::Group(node) => {
                let key =
                    make_state_key(depth, group_number, parent_group_number, child_group_number);
                vec.push((key, current_node.clone()));
                stack.push((node.clone(), depth + 1, group_number));
            }
            EBNFNode::Literal(_) => {
                let key = make_state_key(depth, group_number, parent_group_number, 0);
                vec.push((key, current_node));
            }
        }
    }

    vec.into_iter().collect()
}

fn get_child_count<'a>(node: &EBNFNode<'a>) -> usize {
    match node {
        EBNFNode::Expansion(_) => 0,
        EBNFNode::Or(nodes) => nodes.len(),
        EBNFNode::Concat(nodes) => nodes.len(),
        EBNFNode::Repeat {
            node: _,
            min: _,
            max: _,
        } => 1,
        EBNFNode::Group(_) => 1,
        EBNFNode::Literal(_) => 0,
    }
}
