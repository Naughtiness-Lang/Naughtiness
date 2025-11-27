use std::collections::HashMap;
use std::rc::Rc;

use crate::ebnf_parser::LITERAL_QUOTE;

pub type EBNFStateKey = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct EBNFState {
    depth: u16,
    group: u16,
    parent_group: u16,
    children_group: u16,
}

impl EBNFState {
    fn make_state_key(depth: EBNFStateKey, group: EBNFStateKey) -> Self {
        Self {
            depth,
            group,
            parent_group: 0,
            children_group: 0,
        }
    }

    fn to_state_key(&self) -> EBNFState {
        Self {
            depth: self.depth,
            group: self.group,
            parent_group: 0,
            children_group: 0,
        }
    }

    pub fn root() -> Self {
        Self::make_state_key(0, 0)
    }
}

#[derive(Debug)]
pub(crate) struct EBNF<'a> {
    pub name: &'a str,                               // 定義したルール名
    expr: Rc<EBNFNode<'a>>,                          // ツリー構造(ルールの中身)
    state_map: HashMap<EBNFState, Rc<EBNFNode<'a>>>, // ルールの位置(状態)に応じたマップ
    full_state_map: HashMap<EBNFState, EBNFState>,   // 部分的な状態キーから完全な状態へのマップ
    name_map: HashMap<String, Vec<EBNFState>>,       // ルール名から状態を取得するためのマップ
}

impl<'a> EBNF<'a> {
    pub fn new(name: &'a str, expr: EBNFNode<'a>) -> Self {
        let expr = Rc::new(expr);
        let state_list = make_state_key_pair_list(&expr);
        let mut state_map = HashMap::new();
        let mut full_state_map = HashMap::new();
        let mut name_map = HashMap::new();
        for (full_state, value) in state_list {
            let state_key = full_state.to_state_key();
            name_map
                .entry(get_rule_name(&value))
                .or_insert(vec![])
                .push(state_key);
            state_map.insert(state_key, value);
            full_state_map.insert(state_key, full_state);
        }

        Self {
            name,
            expr,
            state_map,
            full_state_map,
            name_map,
        }
    }

    pub fn get_node(&self, state: &EBNFState) -> Option<&EBNFNode<'a>> {
        self.state_map.get(state).map(|node| &**node)
    }

    pub fn root(&self) -> EBNFState {
        EBNFState::make_state_key(0, 0)
    }

    // 親ノードに移動
    pub fn parent(&self, state: EBNFState) -> Option<EBNFState> {
        let full_state = EBNFState::from(*self.full_state_map.get(&state)?);
        if full_state.depth == 0 {
            return None;
        }
        let parent_state = EBNFState::make_state_key(full_state.depth - 1, full_state.parent_group);
        Some(parent_state)
    }

    // 隣のノードへ移動
    // 同じグループの末端であれば親ノードの隣のノードへ
    // その親が末端であればさらにその親を見ていく
    pub fn next_group(&self, state: EBNFState) -> Option<EBNFState> {
        let full_state = EBNFState::from(*self.full_state_map.get(&state)?);
        let next_group = full_state.group + 1;

        // 単純に1加算した値が次のグループかを判断できないので
        // 親を参照し親のデータから次のグループに移動するかを判断する
        let Some(parent_state) = self.parent(state) else {
            // 親はいないが次のグループが存在する場合
            let next_state = EBNFState::make_state_key(full_state.depth, next_group);
            return if self.state_map.contains_key(&next_state) {
                Some(next_state)
            } else {
                None
            };
        };

        // 次のグループがない場合親ノードへ
        let parent_state = EBNFState::from(parent_state);
        let full_parent_state = EBNFState::from(*self.full_state_map.get(&parent_state)?);
        let parent_node = self.state_map.get(&parent_state)?;
        let count = get_child_count(parent_node) as u16;
        let group_limit = full_parent_state.children_group + count;
        if group_limit < next_group + 1 {
            // グループが存在しない場合はさらに親を見る
            return self.next_group(parent_state);
        }

        Some(EBNFState::make_state_key(full_state.depth, next_group))
    }

    // 子のノードに移動する
    // 子ノードがなければ同グループの隣のノードへ
    // グループの末端かつ子ノードがなければ親ノードへ
    pub fn step_in(&self, state: EBNFState) -> Option<(&EBNFNode<'a>, EBNFState)> {
        let full_state = EBNFState::from(*self.full_state_map.get(&state)?);
        let child_state =
            EBNFState::make_state_key(full_state.depth + 1, full_state.children_group);
        let node = self.get_node(&state)?;
        if get_child_count(node) == 0 {
            return self.step_over(state); // 子がいない場合はstep_overと同じ
        }
        let child_state = child_state.to_state_key();
        let node = self.state_map.get(&child_state)?;

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

fn make_state_key_pair_list<'a>(expr: &Rc<EBNFNode<'a>>) -> Vec<(EBNFState, Rc<EBNFNode<'a>>)> {
    let mut group_count_map = HashMap::new();
    let mut vec = vec![];
    let mut stack = vec![(expr.clone(), 0, 0)];
    while let Some((current_node, depth, parent_group)) = stack.pop() {
        let group = *group_count_map
            .entry(depth)
            .and_modify(|e| *e += 1)
            .or_insert(0);
        let children_group = group_count_map
            .get(&(depth + 1))
            .map(|e| *e + 1)
            .unwrap_or(0);

        match &*current_node {
            EBNFNode::Expansion(_) => {
                let key = EBNFState {
                    depth,
                    group,
                    parent_group,
                    children_group: 0,
                };
                vec.push((key, current_node));
            }
            EBNFNode::Concat(nodes) | EBNFNode::Or(nodes) => {
                let key = EBNFState {
                    depth,
                    group,
                    parent_group,
                    children_group,
                };
                vec.push((key, current_node.clone()));

                for node in nodes.iter().rev() {
                    stack.push((node.clone(), depth + 1, group));
                }
            }
            EBNFNode::Repeat {
                node,
                min: _,
                max: _,
            } => {
                let key = EBNFState {
                    depth,
                    group,
                    parent_group,
                    children_group,
                };
                vec.push((key, current_node.clone()));
                stack.push((node.clone(), depth + 1, group));
            }
            EBNFNode::Group(node) => {
                let key = EBNFState {
                    depth,
                    group,
                    parent_group,
                    children_group,
                };
                vec.push((key, current_node.clone()));
                stack.push((node.clone(), depth + 1, group));
            }
            EBNFNode::Literal(_) => {
                let key = EBNFState {
                    depth,
                    group,
                    parent_group,
                    children_group: 0,
                };
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

pub(crate) fn get_rule_name<'a>(node: &EBNFNode<'a>) -> String {
    match node {
        EBNFNode::Expansion(e) => e.to_string(),
        EBNFNode::Concat(nodes) => nodes
            .iter()
            .map(|node| get_rule_name(node))
            .collect::<Vec<_>>()
            .join(" "),
        EBNFNode::Or(nodes) => nodes
            .iter()
            .map(|node| get_rule_name(node))
            .collect::<Vec<_>>()
            .join(" | "),
        EBNFNode::Group(node) => format!("( {} )", get_rule_name(node)),
        EBNFNode::Repeat { node, min, max } => {
            let rule = get_rule_name(node);
            match (*min, *max) {
                (0, None) => format!("{rule}*"),
                (1, None) => format!("{rule}+"),
                (0, Some(1)) => format!("{rule}?"),
                (min, None) => format!("{rule}{{{min},}}"),
                (min, Some(max)) if min == max => format!("{rule}{{{min}}}"),
                (min, Some(max)) => format!("{rule}{{{min}, {max}}}"),
            }
        }
        EBNFNode::Literal(l) => format!("{LITERAL_QUOTE}{l}{LITERAL_QUOTE}"),
    }
}
