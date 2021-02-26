#ifndef LIB_RUBY_PARSER_GEN_NODES_VARIANT_H
#define LIB_RUBY_PARSER_GEN_NODES_VARIANT_H

#include <memory>
#include <vector>
#include <string>
#include <variant>
#include "classes.h"

namespace lib_ruby_parser {

using node_variant_t = std::variant<
    std::unique_ptr<Alias>,
    std::unique_ptr<AndAsgn>,
    std::unique_ptr<And>,
    std::unique_ptr<Arg>,
    std::unique_ptr<Args>,
    std::unique_ptr<Array>,
    std::unique_ptr<ArrayPattern>,
    std::unique_ptr<ArrayPatternWithTail>,
    std::unique_ptr<BackRef>,
    std::unique_ptr<Begin>,
    std::unique_ptr<Block>,
    std::unique_ptr<BlockPass>,
    std::unique_ptr<Blockarg>,
    std::unique_ptr<Break>,
    std::unique_ptr<Case>,
    std::unique_ptr<CaseMatch>,
    std::unique_ptr<Casgn>,
    std::unique_ptr<Cbase>,
    std::unique_ptr<Class>,
    std::unique_ptr<Complex>,
    std::unique_ptr<Const>,
    std::unique_ptr<ConstPattern>,
    std::unique_ptr<CSend>,
    std::unique_ptr<Cvar>,
    std::unique_ptr<Cvasgn>,
    std::unique_ptr<Def>,
    std::unique_ptr<Defined>,
    std::unique_ptr<Defs>,
    std::unique_ptr<Dstr>,
    std::unique_ptr<Dsym>,
    std::unique_ptr<EFlipFlop>,
    std::unique_ptr<EmptyElse>,
    std::unique_ptr<Encoding>,
    std::unique_ptr<Ensure>,
    std::unique_ptr<Erange>,
    std::unique_ptr<False>,
    std::unique_ptr<File>,
    std::unique_ptr<FindPattern>,
    std::unique_ptr<Float>,
    std::unique_ptr<For>,
    std::unique_ptr<ForwardArg>,
    std::unique_ptr<ForwardedArgs>,
    std::unique_ptr<Gvar>,
    std::unique_ptr<Gvasgn>,
    std::unique_ptr<Hash>,
    std::unique_ptr<Kwargs>,
    std::unique_ptr<HashPattern>,
    std::unique_ptr<Heredoc>,
    std::unique_ptr<If>,
    std::unique_ptr<IfGuard>,
    std::unique_ptr<IfMod>,
    std::unique_ptr<IfTernary>,
    std::unique_ptr<IFlipFlop>,
    std::unique_ptr<MatchPattern>,
    std::unique_ptr<MatchPatternP>,
    std::unique_ptr<InPattern>,
    std::unique_ptr<Index>,
    std::unique_ptr<IndexAsgn>,
    std::unique_ptr<Int>,
    std::unique_ptr<Irange>,
    std::unique_ptr<Ivar>,
    std::unique_ptr<Ivasgn>,
    std::unique_ptr<Kwarg>,
    std::unique_ptr<KwBegin>,
    std::unique_ptr<Kwnilarg>,
    std::unique_ptr<Kwoptarg>,
    std::unique_ptr<Kwrestarg>,
    std::unique_ptr<Kwsplat>,
    std::unique_ptr<Lambda>,
    std::unique_ptr<Line>,
    std::unique_ptr<Lvar>,
    std::unique_ptr<Lvasgn>,
    std::unique_ptr<Masgn>,
    std::unique_ptr<MatchAlt>,
    std::unique_ptr<MatchAs>,
    std::unique_ptr<MatchCurrentLine>,
    std::unique_ptr<MatchNilPattern>,
    std::unique_ptr<MatchRest>,
    std::unique_ptr<MatchVar>,
    std::unique_ptr<MatchWithLvasgn>,
    std::unique_ptr<Mlhs>,
    std::unique_ptr<Module>,
    std::unique_ptr<Next>,
    std::unique_ptr<Nil>,
    std::unique_ptr<NthRef>,
    std::unique_ptr<Numblock>,
    std::unique_ptr<OpAsgn>,
    std::unique_ptr<Optarg>,
    std::unique_ptr<Or>,
    std::unique_ptr<OrAsgn>,
    std::unique_ptr<Pair>,
    std::unique_ptr<Pin>,
    std::unique_ptr<Postexe>,
    std::unique_ptr<Preexe>,
    std::unique_ptr<Procarg0>,
    std::unique_ptr<Rational>,
    std::unique_ptr<Redo>,
    std::unique_ptr<RegOpt>,
    std::unique_ptr<Regexp>,
    std::unique_ptr<Rescue>,
    std::unique_ptr<RescueBody>,
    std::unique_ptr<Restarg>,
    std::unique_ptr<Retry>,
    std::unique_ptr<Return>,
    std::unique_ptr<SClass>,
    std::unique_ptr<Self_>,
    std::unique_ptr<Send>,
    std::unique_ptr<Shadowarg>,
    std::unique_ptr<Splat>,
    std::unique_ptr<Str>,
    std::unique_ptr<Super>,
    std::unique_ptr<Sym>,
    std::unique_ptr<True>,
    std::unique_ptr<Undef>,
    std::unique_ptr<UnlessGuard>,
    std::unique_ptr<Until>,
    std::unique_ptr<UntilPost>,
    std::unique_ptr<When>,
    std::unique_ptr<While>,
    std::unique_ptr<WhilePost>,
    std::unique_ptr<XHeredoc>,
    std::unique_ptr<Xstr>,
    std::unique_ptr<Yield>,
    std::unique_ptr<ZSuper>>;
}
#endif // LIB_RUBY_PARSER_GEN_NODES_VARIANT_H
