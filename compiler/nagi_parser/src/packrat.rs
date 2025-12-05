use crate::{
    ebnf::{EBNFNode, EBNFState, EBNF},
    ebnf_parser::parse_ebnf,
    errors::{PackratError, ParsingError},
    lexer::Lexer,
    Position, Rule,
};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
    thread::panicking,
};

pub(crate) type ParseResult<A, P> = Result<Rc<ASTAssembly<A, P>>, ParsingError>;
pub(crate) type LiteralParser<T, A, P> = fn(literal: &str, token: &T) -> ParseResult<A, P>;
pub(crate) type Rules<'a, T, A, P> = Vec<Result<EBNFTable<'a, T, A, P>, PackratError>>;
type RulesMap<'a, T, A, P> = HashMap<Rule<'a>, Rc<EBNFTable<'a, T, A, P>>>;

#[derive(Debug)]
pub enum ASTBuilder<'a, T, A, P> {
    None,                                         // 定義のみ(ASTの構築を行わない)
    ASTBuild(fn(&EBNF<'a>) -> ParseResult<A, P>), // 解析成功時に呼び出す(AST構築)
    Parsed(fn(&T) -> ParseResult<A, P>),          // トークナイズ時点で解析済みの結果を返す
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTAssembly<A, P> {
    None,         // 必ずASTを持つとは限らない
    AST(Rc<A>),   // AST本体
    Parts(Rc<P>), // 部分的に解析した場合ASTを完全に構築できないので
}

#[derive(Debug, Clone, PartialEq)]
enum MemoResult<A, P> {
    Fail,                         // 保留
    Match(Rc<ASTAssembly<A, P>>), // 解析成功(必ずASTを持つとは限らない)
    MisMatch,                     // 解析失敗
}

#[derive(Debug, Clone)]
struct MemoEntry<A, P> {
    result: Rc<MemoResult<A, P>>, // 解析結果
    position: Position,           // 次の解析位置
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MemoKey<'a> {
    rule_name: Rule<'a>,
    position: Position,
    state: EBNFState,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GrowKey<'a> {
    rule_name: Rule<'a>,
    state: EBNFState,
}

#[derive(Debug)]
struct MemoTable<'a, A, P> {
    table: HashMap<MemoKey<'a>, MemoEntry<A, P>>,
    ast_table: HashMap<Rule<'a>, Vec<MemoKey<'a>>>,
}

#[derive(Debug)]
struct EvalResult<'a, A, P> {
    key: MemoKey<'a>,
    result: Rc<MemoResult<A, P>>,
}

#[derive(Debug)]
pub(crate) struct EBNFTable<'a, T, A, P> {
    parsed_rule: EBNF<'a>,
    ast_builder: ASTBuilder<'a, T, A, P>,
}

impl<'a, T, A, P> EBNFTable<'a, T, A, P> {
    pub(crate) fn new(
        rule: &'a str,
        ast_builder: ASTBuilder<'a, T, A, P>,
    ) -> Result<Self, PackratError> {
        Ok(Self {
            parsed_rule: parse_ebnf(rule)?,
            ast_builder,
        })
    }
}

// これstaticか何かで持ちたい
#[derive(Debug)]
struct DefaultValues<A, P> {
    ast_assembly: Rc<ASTAssembly<A, P>>,
    memo_result_fail: Rc<MemoResult<A, P>>,
    memo_result_mismatch: Rc<MemoResult<A, P>>,
    memo_result_match: Rc<MemoResult<A, P>>,
}

impl<A, P> Default for DefaultValues<A, P> {
    fn default() -> Self {
        let ast_assembly = Rc::new(ASTAssembly::None);
        Self {
            ast_assembly: ast_assembly.clone(),
            memo_result_fail: Rc::new(MemoResult::Fail),
            memo_result_mismatch: Rc::new(MemoResult::MisMatch),
            memo_result_match: Rc::new(MemoResult::Match(ast_assembly.clone())),
        }
    }
}

// T = Token
// A = AST
// P = ASTを構築するためのパーツ
#[derive(Debug)]
pub(crate) struct PackratParser<'a, T, A, P> {
    memo: MemoTable<'a, A, P>,
    grow_list: HashSet<GrowKey<'a>>,
    rules: RulesMap<'a, T, A, P>,
    call_stack: Vec<MemoKey<'a>>,
    literal_parser: LiteralParser<T, A, P>,
    default: DefaultValues<A, P>,
    frame_stack: Vec<Frame<'a, A, P>>,
    eval_result_stack: Vec<EvalResult<'a, A, P>>,
}

#[derive(Debug)]
enum Frame<'a, A, P> {
    ApplyRule {
        key: MemoKey<'a>,
    },
    UpdateMemo {
        key: MemoKey<'a>,
    },
    GrowLRStart {
        key: MemoKey<'a>,
        memo: MemoEntry<A, P>,
    },
    GrowLRStep {
        key: MemoKey<'a>,
        memo: MemoEntry<A, P>,
    },
    HandleLR {
        key: MemoKey<'a>,
    },
    Eval {
        key: MemoKey<'a>,
    },
    EvalResult {
        key: MemoKey<'a>,
        result: Rc<MemoResult<A, P>>,
    },
    Continuation {
        key: MemoKey<'a>,
        node: ContinuationNode<'a>,
    },
    PopCallStack,
}

#[derive(Debug)]
enum ContinuationNode<'a> {
    Concat {
        next: MemoKey<'a>,
    },
    Or {
        next: MemoKey<'a>,
    },
    Rpeat {
        next: MemoKey<'a>,
        count: u64,
        min: u64,
        max: Option<u64>,
    },
    Group {
        next: MemoKey<'a>,
    },
    Expansion {
        next: MemoKey<'a>,
    },
}

//
// スタックオーバーフロー対策として再帰による解析を行わず
// スタックを用いたPackratParserを実装している
// スタックへの変更により解析途中の状態が失われるので
// メモに事前に解析したEBNFの位置情報を保存して解決を図っている
//
// また左再帰の問題の対処として
// 「左再帰に対応するPackrat Parserの実装」の提案手法を取り入れた
// https://ipsj.ixsq.nii.ac.jp/records/82737
//
// これらを組み合わせたPackratParserを実装している
//

trait PackratParseLeftRecursive<'a, T, A, P>: Eval<'a, T, A, P> {
    fn apply_rule(&mut self, lexer: &mut Lexer<T>, key: MemoKey<'a>) -> Self::Result;

    fn update_memo(&mut self, key: MemoKey<'a>) -> Self::Result;

    fn eval_result(
        &mut self,
        lexer: &mut Lexer<T>,
        key: MemoKey<'a>,
        result: Rc<MemoResult<A, P>>,
    ) -> Self::Result;

    fn grow_lr(
        &mut self,
        lexer: &mut Lexer<T>,
        key: MemoKey<'a>,
        memo: MemoEntry<A, P>,
    ) -> Self::Result;

    fn grow_lr_step(
        &mut self,
        lexer: &mut Lexer<T>,
        key: MemoKey<'a>,
        memo: MemoEntry<A, P>,
    ) -> Self::Result;

    fn handle_lr(&mut self, lexer: &mut Lexer<T>, key: MemoKey<'a>) -> Self::Result;

    fn pop_call_stack(&mut self) -> Self::Result;
}

trait Eval<'a, T, A, P> {
    type Result;

    fn eval(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result;

    fn eval_expansion(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: &MemoKey<'a>,
        expansion: &'a str,
    ) -> Self::Result;

    fn eval_concat(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result;

    fn eval_or(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result;

    fn eval_group(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result;

    fn eval_repeat(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: &MemoKey<'a>,
        min: u64,
        max: Option<u64>,
    ) -> Self::Result;

    fn eval_literal(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: &MemoKey<'a>,
        literal: &str,
    ) -> Self::Result;

    fn continuation(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        node: ContinuationNode<'a>,
    ) -> Self::Result;

    fn continuation_concat(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result;

    fn continuation_or(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result;

    fn continuation_repeat(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
        count: u64,
        min: u64,
        max: Option<u64>,
    ) -> Self::Result;

    fn continuation_group(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result;

    fn continuation_expansion(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result;

    fn parsed_rule(&self, lexer: &mut Lexer<T>, func: &fn(&T) -> ParseResult<A, P>)
        -> Self::Result;
}

impl<'a, T, A, P> PackratParseLeftRecursive<'a, T, A, P> for PackratParser<'a, T, A, P>
where
    T: Debug,
    A: Clone + PartialEq + Debug,
    P: Clone + PartialEq + Debug,
{
    /// ルールの適用
    fn apply_rule(&mut self, lexer: &mut Lexer<T>, key: MemoKey<'a>) -> Self::Result {
        let Some(memo) = self.get_memo(&key).cloned() else {
            self.frame_stack.push(Frame::UpdateMemo { key });
            return Ok(self.default.memo_result_fail.clone());
        };

        // call_stackに含まれているかつ非終端記号であればgrowに追加
        if !self.call_stack.contains(&key) {
            self.frame_stack.push(Frame::UpdateMemo { key });
            lexer.set_token_position(memo.position);
            return Ok(self.default.memo_result_fail.clone());
        }

        let ebnf = self.get_ebnf(&key)?;
        let node = ebnf.parsed_rule.get_node(&key.state);
        let is_term = matches!(node, Some(EBNFNode::Literal(_)))
            || matches!(ebnf.ast_builder, ASTBuilder::Parsed(_));
        if matches!(*memo.result, MemoResult::Fail) && !is_term {
            self.grow_list.insert(GrowKey {
                rule_name: key.rule_name,
                state: key.state,
            });
        }

        lexer.set_token_position(memo.position);

        // メモ化された結果を使用(再評価は行わない)
        Ok(memo.result.clone())
    }

    /// メモの更新
    fn update_memo(&mut self, key: MemoKey<'a>) -> Self::Result {
        self.call_stack.push(key.clone());

        // メモが存在しなければ更新
        // メモが存在すれば情報取得
        self.memo.table.entry(key.clone()).or_insert(MemoEntry {
            result: self.default.memo_result_fail.clone(),
            position: key.position,
        });

        self.frame_stack.push(Frame::PopCallStack); // 15
        self.frame_stack.push(Frame::HandleLR { key: key.clone() }); // 12-14
        self.frame_stack.push(Frame::Eval { key }); // 7-11

        Ok(self.default.memo_result_fail.clone())
    }

    // 評価の結果
    fn eval_result(
        &mut self,
        lexer: &mut Lexer<T>,
        key: MemoKey<'a>,
        result: Rc<MemoResult<A, P>>,
    ) -> Self::Result {
        // 結果がMatchかつルートノードの場合は
        // そのルールはすべて解析完了したということなので
        // AST構築関数を呼び出す
        let result = if matches!(*result, MemoResult::Match(_)) && key.state == EBNFState::root() {
            self.construct_ast(result, &key)?
        } else {
            result
        };

        println!("result_key: {key:?}");
        println!("result: {result:?} <- {key:?}");
        self.eval_result_stack.push(EvalResult { key, result });

        Ok(self.default.memo_result_fail.clone())
    }

    // 8-
    fn handle_lr(&mut self, lexer: &mut Lexer<T>, key: MemoKey<'a>) -> Self::Result {
        let Some(eval_result) = self.eval_result_stack.pop() else {
            return Err(PackratError::InvalidState);
        };
        if eval_result.key != key {
            panic!("");
        }

        // 8-9
        let Some(mut memo) = self.memo.table.get(&key).cloned() else {
            return Err(PackratError::InvalidState);
        };

        let position = lexer.get_token_position();
        if matches!(*eval_result.result, MemoResult::Match(_)) {
            memo.position = position;
        }

        // 10-11
        if memo.position != position {
            memo.result = eval_result.result.clone();
        }

        self.save_memo(
            key.clone(),
            MemoEntry {
                result: eval_result.result.clone(),
                position: lexer.get_token_position(),
            },
        );
        //memo.result = result.clone();

        // 12-14
        // growに含まれていて解析結果が解析前より進んでいた場合は
        // 可能な限り成長させる
        let gk = GrowKey {
            rule_name: key.rule_name,
            state: key.state,
        };
        if self.grow_list.contains(&gk) && key.position < memo.position {
            self.frame_stack.push(Frame::GrowLRStart { key, memo });
        }

        Ok(self.default.memo_result_fail.clone())
    }

    // 左再帰を含む構文であれば可能な限り成長させる
    fn grow_lr(
        &mut self,
        lexer: &mut Lexer<T>,
        key: MemoKey<'a>,
        memo: MemoEntry<A, P>,
    ) -> Self::Result {
        while let Some(last) = self.call_stack.last() {
            if *last == key {
                break;
            }
            self.call_stack.pop();
        }

        // 評価を開始
        // 6
        lexer.set_token_position(key.position);

        // 7
        self.frame_stack.push(Frame::GrowLRStep {
            key: key.clone(),
            memo,
        });
        self.frame_stack.push(Frame::Eval { key });

        Ok(self.default.memo_result_fail.clone())
    }

    fn grow_lr_step(
        &mut self,
        lexer: &mut Lexer<T>,
        key: MemoKey<'a>,
        mut memo: MemoEntry<A, P>,
    ) -> Self::Result {
        let Some(eval_result) = self.eval_result_stack.pop() else {
            return Err(PackratError::InvalidState);
        };
        if eval_result.key != key {
            panic!("");
        }

        // 8-9
        // latest_memo.positionが(最新解析後の位置)
        // メモの結果がFailなら成長不可でループ終端
        // break文と等価
        let position = lexer.get_token_position();
        if matches!(*eval_result.result, MemoResult::Fail) || position < memo.position + 1 {
            // 14
            lexer.set_token_position(memo.position);

            // 15の代わり(値を返しても意味がないので)
            // 結果をメモに反映
            self.save_memo(key.clone(), memo.clone());

            return Ok(self.default.memo_result_fail.clone());
        }

        eprintln!("grow! {key:?}");

        // 10-11
        memo.result = eval_result.result.clone();
        memo.position = lexer.get_token_position();

        // 再評価のために先にフレームを積む(ループを継続)
        self.frame_stack.push(Frame::GrowLRStep {
            key: key.clone(),
            memo,
        });

        // 再評価を開始
        // 6-7
        lexer.set_token_position(key.position);
        self.frame_stack.push(Frame::Eval { key });

        Ok(self.default.memo_result_fail.clone())
    }

    fn pop_call_stack(&mut self) -> Self::Result {
        let a = self.call_stack.pop();
        println!("pop: {a:?}");
        Ok(self.default.memo_result_fail.clone())
    }
}

impl<'a, T, A, P> Eval<'a, T, A, P> for PackratParser<'a, T, A, P>
where
    T: Debug,
    A: Clone + PartialEq + Debug,
    P: Clone + PartialEq + Debug,
{
    type Result = Result<Rc<MemoResult<A, P>>, PackratError>;

    fn eval(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result {
        let ebnf = self.get_ebnf(rule)?;
        let Some(node) = ebnf.parsed_rule.get_node(&rule.state) else {
            return Err(PackratError::InvalidState);
        };

        /*
        println!(
            "{} ::= {}",
            ebnf.parsed_rule.name,
            ebnf.parsed_rule.get_node_name(&EBNFState::root())
        );
        println!("parse: {}", ebnf.parsed_rule.get_node_name(&rule.state));
         * */

        let result = match node {
            EBNFNode::Expansion(expansion) => self.eval_expansion(lexer, rule, expansion)?,
            EBNFNode::Concat(_) => self.eval_concat(lexer, rule)?,
            EBNFNode::Or(_) => self.eval_or(lexer, rule)?,
            EBNFNode::Repeat { node: _, min, max } => self.eval_repeat(lexer, rule, *min, *max)?,
            EBNFNode::Group(_) => self.eval_group(lexer, rule)?,
            EBNFNode::Literal(literal) => self.eval_literal(lexer, rule, literal)?,
        };

        println!("result: {result:?} <- {:?}", ebnf.parsed_rule.name);

        Ok(result)
    }

    /// 事前に解析済みのトークンであればその結果を返す
    /// ルールの本体の一番最初に解析するルールを見て
    /// 末端のルールもしくは繰り返しルールになるまでcall_stackに積む
    ///
    fn eval_expansion(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: &MemoKey<'a>,
        expansion: &'a str,
    ) -> Self::Result {
        let Some(table) = self.rules.get(expansion).cloned() else {
            return Err(PackratError::UnknownRule {
                unknown_rule_name: expansion.to_string(),
                call_rule: rule.rule_name.to_string(),
            });
        };

        // 解析済みのトークンの場合はここで結果をもらう
        if let ASTBuilder::Parsed(func) = &table.ast_builder {
            self.frame_stack.push(Frame::EvalResult {
                key: rule.clone(),
                result: self.parsed_rule(lexer, func)?,
            });

            return Ok(self.default.memo_result_fail.clone());
        }

        let next = MemoKey {
            rule_name: table.parsed_rule.name,
            position: rule.position,
            state: EBNFState::root(),
        };

        self.frame_stack.push(Frame::Continuation {
            key: rule.clone(),
            node: ContinuationNode::Expansion { next: next.clone() },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    // すべての子ノードがMatchして初めてMatchを返す
    fn eval_concat(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result {
        let (_, next_state) = self.get_ebnf_and_child_state(rule)?;
        let next = MemoKey {
            rule_name: rule.rule_name,
            position: rule.position,
            state: next_state,
        };

        self.frame_stack.push(Frame::Continuation {
            key: rule.clone(),
            node: ContinuationNode::Concat { next: next.clone() },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    /// 子ノードを順に見ていく
    /// 貪欲に見る場合は全てを評価して最も解析が進んだノードを返す
    /// 貪欲に見ない場合は最初にMatchしたノードを返す
    fn eval_or(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result {
        let (_, next_state) = self.get_ebnf_and_child_state(rule)?;
        let next = MemoKey {
            rule_name: rule.rule_name,
            position: rule.position,
            state: next_state,
        };

        self.frame_stack.push(Frame::Continuation {
            key: rule.clone(),
            node: ContinuationNode::Or { next: next.clone() },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    fn eval_group(&mut self, lexer: &mut Lexer<T>, rule: &MemoKey<'a>) -> Self::Result {
        let (_, next_state) = self.get_ebnf_and_child_state(rule)?;
        let next = MemoKey {
            rule_name: rule.rule_name,
            position: rule.position,
            state: next_state,
        };

        self.frame_stack.push(Frame::Continuation {
            key: rule.clone(),
            node: ContinuationNode::Group { next: next.clone() },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    fn eval_repeat(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: &MemoKey<'a>,
        min: u64,
        max: Option<u64>,
    ) -> Self::Result {
        let (_, next_state) = self.get_ebnf_and_child_state(rule)?;
        let next = MemoKey {
            rule_name: rule.rule_name,
            position: rule.position,
            state: next_state,
        };

        self.frame_stack.push(Frame::Continuation {
            key: rule.clone(),
            node: ContinuationNode::Rpeat {
                next: next.clone(),
                count: 0,
                min,
                max,
            },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    /// 末端なのでFailを返すことはない
    fn eval_literal(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: &MemoKey<'a>,
        literal: &str,
    ) -> Self::Result {
        let Some(token) = lexer.peek() else {
            return Err(PackratError::UnexpectedEOF);
        };

        let result = match (self.literal_parser)(literal, token) {
            Ok(ast) => {
                lexer.next();
                Rc::new(MemoResult::Match(ast))
            }
            Err(_) => self.default.memo_result_mismatch.clone(),
        };

        self.frame_stack.push(Frame::EvalResult {
            key: rule.clone(),
            result,
        });

        Ok(self.default.memo_result_fail.clone())
    }

    fn continuation(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        node: ContinuationNode<'a>,
    ) -> Self::Result {
        match node {
            ContinuationNode::Expansion { next } => self.continuation_expansion(lexer, rule, next),
            ContinuationNode::Concat { next } => self.continuation_concat(lexer, rule, next),
            ContinuationNode::Or { next } => self.continuation_or(lexer, rule, next),
            ContinuationNode::Rpeat {
                next,
                count,
                min,
                max,
            } => self.continuation_repeat(lexer, rule, next, count, min, max),
            ContinuationNode::Group { next } => self.continuation_group(lexer, rule, next),
        }
    }

    fn continuation_expansion(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result {
        // メモはある前提
        let Some(memo) = self.get_memo(&next_rule) else {
            return Err(PackratError::NotFoundMemo);
        };
        lexer.set_token_position(memo.position);

        self.frame_stack.push(Frame::EvalResult {
            key: rule,
            result: memo.result.clone(),
        });

        Ok(self.default.memo_result_fail.clone())
    }

    fn continuation_concat(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result {
        let Some(memo) = self.get_memo(&next_rule) else {
            return Err(PackratError::NotFoundMemo);
        };
        lexer.set_token_position(memo.position);

        // 末端まで全てMatchでなければMisMatch
        if matches!(*memo.result, MemoResult::MisMatch) {
            self.frame_stack.push(Frame::EvalResult {
                key: rule,
                result: memo.result.clone(),
            });
            return Ok(self.default.memo_result_fail.clone());
        }

        // 末端まで
        let ebnf = self.get_ebnf(&rule)?;
        let Some((_, next_state)) = ebnf.parsed_rule.next_group(next_rule.state) else {
            let result = self.default.memo_result_match.clone();
            self.frame_stack.push(Frame::EvalResult {
                key: rule,
                result: result.clone(),
            });
            return Ok(result);
        };

        let next = MemoKey {
            rule_name: rule.rule_name,
            position: lexer.get_token_position(),
            state: next_state,
        };

        // 次のノードを評価
        self.frame_stack.push(Frame::Continuation {
            key: rule,
            node: ContinuationNode::Concat { next: next.clone() },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    fn continuation_or(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result {
        let Some(memo) = self.get_memo(&next_rule) else {
            return Err(PackratError::NotFoundMemo);
        };
        lexer.set_token_position(memo.position);

        // いずれか1つでもMatchであればOr全体はMatch
        if matches!(*memo.result, MemoResult::Match(_)) {
            self.frame_stack.push(Frame::EvalResult {
                key: rule,
                result: memo.result.clone(),
            });
            return Ok(self.default.memo_result_fail.clone());
        }

        let ebnf = self.get_ebnf(&rule)?;
        let Some((_, next_state)) = ebnf.parsed_rule.next_group(next_rule.state) else {
            self.frame_stack.push(Frame::EvalResult {
                key: rule,
                result: self.default.memo_result_mismatch.clone(),
            });
            return Ok(self.default.memo_result_fail.clone());
        };

        let next = MemoKey {
            rule_name: rule.rule_name,
            position: rule.position,
            state: next_state,
        };

        // 次のノードを評価
        self.frame_stack.push(Frame::Continuation {
            key: rule,
            node: ContinuationNode::Or { next: next.clone() },
        });
        self.frame_stack.push(Frame::ApplyRule { key: next });

        Ok(self.default.memo_result_fail.clone())
    }

    fn continuation_repeat(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
        count: u64,
        min: u64,
        max: Option<u64>,
    ) -> Self::Result {
        let Some(memo) = self.get_memo(&next_rule) else {
            return Err(PackratError::NotFoundMemo);
        };
        lexer.set_token_position(memo.position);

        // MisMatchでも最低回数より小さくなければMatchとする(minが0ならMisMatchでもMatchになる)
        // maxがNoneの場合は永遠にMatchさせる
        // maxがNoneかつ位置が更新されていない場合は無限ループになる
        match *memo.result {
            MemoResult::MisMatch if count < min => {
                self.frame_stack.push(Frame::EvalResult {
                    key: rule,
                    result: self.default.memo_result_match.clone(),
                });
                return Ok(self.default.memo_result_fail.clone());
            }
            MemoResult::MisMatch => {
                self.frame_stack.push(Frame::EvalResult {
                    key: rule,
                    result: self.default.memo_result_match.clone(),
                });
                return Ok(self.default.memo_result_fail.clone());
            }
            MemoResult::Match(_) if Some(count) == max => {
                self.frame_stack.push(Frame::EvalResult {
                    key: rule,
                    result: memo.result.clone(),
                });
                return Ok(self.default.memo_result_fail.clone());
            }
            MemoResult::Match(_)
                if rule.position == lexer.get_token_position() && max.is_none() =>
            {
                panic!("occurrence infinite loop");
            }
            MemoResult::Match(_) => (),
            MemoResult::Fail => (),
        };

        if rule.position == lexer.get_token_position() && max.is_none() {
            self.frame_stack.push(Frame::EvalResult {
                key: rule,
                result: self.default.memo_result_mismatch.clone(),
            });
            return Ok(self.default.memo_result_fail.clone());
        }

        if Some(count) == max {
            self.frame_stack.push(Frame::EvalResult {
                key: rule,
                result: self.default.memo_result_mismatch.clone(),
            });
            return Ok(self.default.memo_result_fail.clone());
        }

        let next_rule = MemoKey {
            rule_name: next_rule.rule_name,
            position: lexer.get_token_position(),
            state: next_rule.state,
        };

        // 次のノードを評価
        self.frame_stack.push(Frame::Continuation {
            key: rule,
            node: ContinuationNode::Rpeat {
                count: count + 1,
                min,
                max,
                next: next_rule.clone(),
            },
        });

        self.frame_stack.push(Frame::ApplyRule { key: next_rule });

        Ok(self.default.memo_result_fail.clone())
    }

    // 遅延評価
    fn continuation_group(
        &mut self,
        lexer: &mut Lexer<T>,
        rule: MemoKey<'a>,
        next_rule: MemoKey<'a>,
    ) -> Self::Result {
        let Some(memo) = self.get_memo(&next_rule) else {
            return Err(PackratError::NotFoundMemo);
        };
        lexer.set_token_position(memo.position);

        self.frame_stack.push(Frame::EvalResult {
            key: rule,
            result: memo.result.clone(),
        });

        Ok(self.default.memo_result_fail.clone())
    }

    fn parsed_rule(
        &self,
        lexer: &mut Lexer<T>,
        func: &fn(&T) -> ParseResult<A, P>,
    ) -> Self::Result {
        let Some(token) = lexer.peek() else {
            return Err(PackratError::UnexpectedEOF);
        };
        let Ok(ast) = func(token) else {
            return Ok(self.default.memo_result_mismatch.clone());
        };
        lexer.next();
        Ok(Rc::new(MemoResult::Match(ast)))
    }
}

impl<'a, T, A, P> PackratParser<'a, T, A, P>
where
    T: Debug,
    A: Clone + PartialEq + Debug,
    P: Clone + PartialEq + Debug,
{
    pub fn new(
        literal_parser: LiteralParser<T, A, P>,
        rules: Rules<'a, T, A, P>,
    ) -> Result<Self, PackratError> {
        Ok(Self {
            memo: MemoTable {
                table: HashMap::new(),
                ast_table: HashMap::new(),
            },
            grow_list: HashSet::new(),
            rules: make_rules_map(rules)?,
            call_stack: vec![],
            literal_parser,
            default: DefaultValues::default(),
            frame_stack: vec![],
            eval_result_stack: vec![],
        })
    }

    // 先にルールで解析
    // 解析後に対応したAST構築用の関数を呼び出す
    pub fn parse(
        &mut self,
        lexer: &mut Lexer<T>,
        first_rule: Rule<'a>,
    ) -> Result<Rc<A>, PackratError> {
        let first_rule = MemoKey {
            rule_name: first_rule,
            position: lexer.get_token_position(),
            state: EBNFState::root(),
        };

        let start_frame = Frame::ApplyRule {
            key: first_rule.clone(),
        };

        self.frame_stack.push(start_frame);

        while let Some(frame) = self.frame_stack.pop() {
            println!("--------");
            println!("frame: {frame:?}");
            println!("peek: {:?}", lexer.peek());
            match frame {
                Frame::ApplyRule { key } => self.apply_rule(lexer, key)?,
                Frame::UpdateMemo { key } => self.update_memo(key)?,
                Frame::Eval { key } => self.eval(lexer, &key)?,
                Frame::GrowLRStart { key, memo } => self.grow_lr(lexer, key, memo)?,
                Frame::GrowLRStep { key, memo } => self.grow_lr_step(lexer, key, memo)?,
                Frame::HandleLR { key } => self.handle_lr(lexer, key)?,
                Frame::EvalResult { key, result } => self.eval_result(lexer, key, result)?,
                Frame::Continuation { key, node } => self.continuation(lexer, key, node)?,
                Frame::PopCallStack => self.pop_call_stack()?,
            };
        }

        println!("eval: {:?}", self.eval_result_stack);
        println!("call: {:?}", self.call_stack);

        let Some(memo) = self.memo.table.get(&first_rule).cloned() else {
            return Err(PackratError::FailedConstructAST);
        };
        eprintln!("Some Memo!");
        let MemoResult::Match(ast) = memo.result.as_ref() else {
            return Err(PackratError::FailedConstructAST);
        };
        eprintln!("Match!!");
        let ASTAssembly::AST(ast) = ast.as_ref() else {
            return Err(PackratError::FailedConstructAST);
        };
        eprintln!("Ok!!!");

        Ok(ast.clone())
    }

    fn get_ebnf(&self, rule: &MemoKey<'a>) -> Result<Rc<EBNFTable<'a, T, A, P>>, PackratError> {
        self.rules
            .get(rule.rule_name)
            .cloned()
            .ok_or(PackratError::UnknownRule {
                unknown_rule_name: rule.rule_name.to_string(),
                call_rule: self
                    .call_stack
                    .last()
                    .map(|f| f.rule_name.to_string())
                    .unwrap_or("unknown rule was called first".to_string()),
            })
    }

    fn get_ebnf_and_child_state(
        &self,
        rule: &MemoKey<'a>,
    ) -> Result<(Rc<EBNFTable<'a, T, A, P>>, EBNFState), PackratError> {
        let ebnf = self.get_ebnf(rule)?;
        // 子ノードに移動
        let Some((_, next_state)) = ebnf.parsed_rule.step_in(rule.state) else {
            return Err(PackratError::InvalidState);
        };

        Ok((ebnf.clone(), next_state))
    }

    fn construct_ast(
        &self,
        result: Rc<MemoResult<A, P>>,
        rule: &MemoKey<'a>,
    ) -> Result<Rc<MemoResult<A, P>>, PackratError> {
        let ebnf = self.get_ebnf(rule)?;

        if !matches!(*result, MemoResult::Match(_)) {
            return Ok(result);
        }

        println!("Ok: {rule:?}");

        let ASTBuilder::ASTBuild(builder) = ebnf.ast_builder else {
            return Ok(result);
        };

        Ok(Rc::new(MemoResult::Match(
            builder(&ebnf.parsed_rule).map_err(|_| PackratError::FailedConstructAST)?,
        )))
    }

    fn get_key_node_name(&self, rule: &MemoKey<'a>) -> String {
        let Ok(ebnf) = self.get_ebnf(rule) else {
            return String::new();
        };
        ebnf.parsed_rule.get_node_name(&rule.state)
    }

    fn save_memo(&mut self, key: MemoKey<'a>, memo: MemoEntry<A, P>) {
        let Ok(ebnf) = self.get_ebnf(&key) else {
            return;
        };
        let Some(node) = ebnf.parsed_rule.get_node(&key.state) else {
            return;
        };
        if let EBNFNode::Expansion(expansion) = node {
            self.memo.table.insert(
                MemoKey {
                    rule_name: expansion,
                    position: key.position,
                    state: EBNFState::root(),
                },
                memo.clone(),
            );
        }

        self.memo.table.insert(key, memo);
    }

    fn get_memo(&self, key: &MemoKey<'a>) -> Option<&MemoEntry<A, P>> {
        let memo = self.memo.table.get(key)?;

        let ebnf = self.get_ebnf(key).ok()?;
        let node = ebnf.parsed_rule.get_node(&key.state);

        // Expansionの場合はステートが変わる
        let memo = if let Some(EBNFNode::Expansion(expansion)) = node {
            self.memo
                .table
                .get(&MemoKey {
                    rule_name: expansion,
                    position: key.position,
                    state: EBNFState::root(),
                })
                .unwrap_or(memo)
        } else {
            memo
        };

        Some(memo)
    }
}

fn make_rules_map<'a, T, A, P>(
    rules: Rules<'a, T, A, P>,
) -> Result<RulesMap<'a, T, A, P>, PackratError> {
    let rules = rules.into_iter().collect::<Result<Vec<_>, _>>()?;
    let mut hash_map = HashMap::new();
    for table in rules {
        if hash_map.contains_key(table.parsed_rule.name) {
            return Err(PackratError::DuplicateRule(
                table.parsed_rule.name.to_string(),
            ));
        }
        hash_map.insert(table.parsed_rule.name, Rc::new(table));
    }

    Ok(hash_map)
}
