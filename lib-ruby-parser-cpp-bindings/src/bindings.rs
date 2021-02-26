/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct BytePtr {
    pub ptr: *mut ::std::os::raw::c_char,
    pub size: u32,
}
extern "C" {
    pub fn make_byte_ptr(ptr: *const ::std::os::raw::c_char, size: u32) -> BytePtr;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Bytes {
    pub _bindgen_opaque_blob: [u64; 2usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Input {
    pub _bindgen_opaque_blob: u64,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct Loc {
    pub _bindgen_opaque_blob: [u32; 2usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub _bindgen_opaque_blob: [u64; 2usize],
}
#[repr(C)]
pub struct NodeVec {
    pub ptr: *mut *mut Node,
    pub length: u32,
}
extern "C" {
    pub fn make_alias(
        to: *mut Node,
        from: *mut Node,
        keyword_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_and_asgn(
        recv: *mut Node,
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_and(
        lhs: *mut Node,
        rhs: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_arg(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_args(
        args: NodeVec,
        expression_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_array(
        elements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_array_pattern(
        elements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_array_pattern_with_tail(
        elements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_back_ref(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_begin(
        statements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_block(
        call: *mut Node,
        args: *mut Node,
        body: *mut Node,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_block_pass(
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_blockarg(
        name: BytePtr,
        operator_l: *mut Loc,
        name_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_break_(args: NodeVec, keyword_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_case(
        expr: *mut Node,
        when_bodies: NodeVec,
        else_body: *mut Node,
        keyword_l: *mut Loc,
        else_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_case_match(
        expr: *mut Node,
        in_bodies: NodeVec,
        else_body: *mut Node,
        keyword_l: *mut Loc,
        else_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_casgn(
        scope: *mut Node,
        name: BytePtr,
        value: *mut Node,
        double_colon_l: *mut Loc,
        name_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_cbase(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_class(
        name: *mut Node,
        superclass: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        operator_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_complex(value: BytePtr, operator_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_const_(
        scope: *mut Node,
        name: BytePtr,
        double_colon_l: *mut Loc,
        name_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_const_pattern(
        const_: *mut Node,
        pattern: *mut Node,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_csend(
        recv: *mut Node,
        method_name: BytePtr,
        args: NodeVec,
        dot_l: *mut Loc,
        selector_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_cvar(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_cvasgn(
        name: BytePtr,
        value: *mut Node,
        name_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_def(
        name: BytePtr,
        args: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        name_l: *mut Loc,
        end_l: *mut Loc,
        assignment_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_defined(
        value: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_defs(
        definee: *mut Node,
        name: BytePtr,
        args: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        operator_l: *mut Loc,
        name_l: *mut Loc,
        assignment_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_dstr(
        parts: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_dsym(
        parts: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_eflipflop(
        left: *mut Node,
        right: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_empty_else(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_encoding_(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_ensure(
        body: *mut Node,
        ensure: *mut Node,
        keyword_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_erange(
        left: *mut Node,
        right: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_false_(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_file(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_find_pattern(
        elements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_float(value: BytePtr, operator_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_for_(
        iterator: *mut Node,
        iteratee: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        operator_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_forward_arg(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_forwarded_args(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_gvar(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_gvasgn(
        name: BytePtr,
        value: *mut Node,
        name_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_hash(
        pairs: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_kwargs(pairs: NodeVec, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_hash_pattern(
        elements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_heredoc(
        parts: NodeVec,
        heredoc_body_l: *mut Loc,
        heredoc_end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_if_(
        cond: *mut Node,
        if_true: *mut Node,
        if_false: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        else_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_if_guard(cond: *mut Node, keyword_l: *mut Loc, expression_l: *mut Loc)
        -> *mut Node;
}
extern "C" {
    pub fn make_if_mod(
        cond: *mut Node,
        if_true: *mut Node,
        if_false: *mut Node,
        keyword_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_if_ternary(
        cond: *mut Node,
        if_true: *mut Node,
        if_false: *mut Node,
        question_l: *mut Loc,
        colon_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_iflipflop(
        left: *mut Node,
        right: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_pattern(
        value: *mut Node,
        pattern: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_pattern_p(
        value: *mut Node,
        pattern: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_in_pattern(
        pattern: *mut Node,
        guard: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_index(
        recv: *mut Node,
        indexes: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_index_asgn(
        recv: *mut Node,
        indexes: NodeVec,
        value: *mut Node,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_int(value: BytePtr, operator_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_irange(
        left: *mut Node,
        right: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_ivar(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_ivasgn(
        name: BytePtr,
        value: *mut Node,
        name_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_kwarg(name: BytePtr, name_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_kwbegin(
        statements: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_kwnilarg(name_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_kwoptarg(
        name: BytePtr,
        default_: *mut Node,
        name_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_kwrestarg(
        name: BytePtr,
        operator_l: *mut Loc,
        name_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_kwsplat(
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_lambda(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_line(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_lvar(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_lvasgn(
        name: BytePtr,
        value: *mut Node,
        name_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_masgn(
        lhs: *mut Node,
        rhs: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_alt(
        lhs: *mut Node,
        rhs: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_as(
        value: *mut Node,
        as_: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_current_line(re: *mut Node, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_match_nil_pattern(
        operator_l: *mut Loc,
        name_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_rest(
        name: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_match_var(name: BytePtr, name_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_match_with_lvasgn(
        re: *mut Node,
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_mlhs(
        items: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_module(
        name: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_next(args: NodeVec, keyword_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_nil(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_nth_ref(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_numblock(
        call: *mut Node,
        numargs: u32,
        body: *mut Node,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_op_asgn(
        recv: *mut Node,
        operator_: BytePtr,
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_optarg(
        name: BytePtr,
        default_: *mut Node,
        name_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_or(
        lhs: *mut Node,
        rhs: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_or_asgn(
        recv: *mut Node,
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_pair(
        key: *mut Node,
        value: *mut Node,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_pin(var: *mut Node, selector_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_postexe(
        body: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_preexe(
        body: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_procarg0(
        args: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_rational(value: BytePtr, operator_l: *mut Loc, expression_l: *mut Loc)
        -> *mut Node;
}
extern "C" {
    pub fn make_redo(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_reg_opt(options: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_regexp(
        parts: NodeVec,
        options: *mut Node,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_rescue(
        body: *mut Node,
        rescue_bodies: NodeVec,
        else_: *mut Node,
        else_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_rescue_body(
        exc_list: *mut Node,
        exc_var: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        assoc_l: *mut Loc,
        begin_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_restarg(
        name: BytePtr,
        operator_l: *mut Loc,
        name_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_retry(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_return_(args: NodeVec, keyword_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_sclass(
        expr: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        operator_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_self_(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_send(
        recv: *mut Node,
        method_name: BytePtr,
        args: NodeVec,
        dot_l: *mut Loc,
        selector_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        operator_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_shadowarg(name: BytePtr, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_splat(value: *mut Node, operator_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_str_(
        value: BytePtr,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_super_(
        args: NodeVec,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_sym(
        name: BytePtr,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_true_(expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_undef(names: NodeVec, keyword_l: *mut Loc, expression_l: *mut Loc) -> *mut Node;
}
extern "C" {
    pub fn make_unless_guard(
        cond: *mut Node,
        keyword_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_until(
        cond: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_until_post(
        cond: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_when(
        patterns: NodeVec,
        body: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_while_(
        cond: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_while_post(
        cond: *mut Node,
        body: *mut Node,
        keyword_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_x_heredoc(
        parts: NodeVec,
        heredoc_body_l: *mut Loc,
        heredoc_end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_xstr(
        parts: NodeVec,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_yield_(
        args: NodeVec,
        keyword_l: *mut Loc,
        begin_l: *mut Loc,
        end_l: *mut Loc,
        expression_l: *mut Loc,
    ) -> *mut Node;
}
extern "C" {
    pub fn make_zsuper(expression_l: *mut Loc) -> *mut Node;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorLevel {
    WARNING = 0,
    ERROR = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Diagnostic {
    _unused: [u8; 0],
}
extern "C" {
    pub fn make_fraction_after_numeric(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_no_digits_after_dot(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unknown_type_of_percent_string(level: ErrorLevel, loc: *mut Loc)
        -> *mut Diagnostic;
}
extern "C" {
    pub fn make_numeric_literal_without_digits(level: ErrorLevel, loc: *mut Loc)
        -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_list(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_regexp(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_string(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_quoted_string(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_unicode_escape(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_too_large_unicode_codepoint(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_unicode_codepoint(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_multiple_codepoint_at_single_char(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_escape_character(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_hex_escape(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_heredoc(
        level: ErrorLevel,
        loc: *mut Loc,
        heredoc_id: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_heredoc_id(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_slash_r_at_middle_of_line(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_d_star_interpreted_as_arg_prefix(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_star_interpreted_as_arg_prefix(level: ErrorLevel, loc: *mut Loc)
        -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ampersand_interpreted_as_arg_prefix(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_triple_dot_at_eol(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_parentheses_iterpreted_as_arglist(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ambiguous_first_argument(
        level: ErrorLevel,
        loc: *mut Loc,
        operator_: ::std::os::raw::c_char,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ambiguous_operator(
        level: ErrorLevel,
        loc: *mut Loc,
        operator_: BytePtr,
        interpreted_as: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_character_syntax(
        level: ErrorLevel,
        loc: *mut Loc,
        suggestion: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_octal_digit(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_trailing_char_in_number(
        level: ErrorLevel,
        loc: *mut Loc,
        c: ::std::os::raw::c_char,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_embedded_document_meets_eof(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_char(
        level: ErrorLevel,
        loc: *mut Loc,
        c: ::std::os::raw::c_char,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_incomplete_character_syntax(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_gvar_without_id(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_gvar_name(
        level: ErrorLevel,
        loc: *mut Loc,
        c: ::std::os::raw::c_char,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ivar_without_id(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_ivar_name(
        level: ErrorLevel,
        loc: *mut Loc,
        c: ::std::os::raw::c_char,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cvar_without_id(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_cvar_name(
        level: ErrorLevel,
        loc: *mut Loc,
        c: ::std::os::raw::c_char,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unknown_regex_options(
        level: ErrorLevel,
        loc: *mut Loc,
        options: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unterminated_unicode_escape(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_encoding_error(level: ErrorLevel, loc: *mut Loc, error: BytePtr)
        -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ambiguous_ternary_operator(
        level: ErrorLevel,
        loc: *mut Loc,
        condition: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ambiguous_regexp(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_else_without_rescue(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_begin_not_at_top_level(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_alias_nth_ref(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_csend_inside_masgn(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_class_or_module_name_must_be_constant(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_endless_setter_definition(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_unexpected_token(
        level: ErrorLevel,
        loc: *mut Loc,
        token_name: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_class_definition_in_method_body(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_module_definition_in_method_body(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_return_in_class_or_module_body(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_const_argument(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ivar_argument(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_gvar_argument(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cvar_argument(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_no_such_local_variable(
        level: ErrorLevel,
        loc: *mut Loc,
        var_name: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_ordinary_param_defined(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_numparam_used(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_tok_at_eol_without_expression(
        level: ErrorLevel,
        loc: *mut Loc,
        token_name: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_end_in_method(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_comparison_after_comparison(
        level: ErrorLevel,
        loc: *mut Loc,
        comparison: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_circular_argument_reference(
        level: ErrorLevel,
        loc: *mut Loc,
        arg_name: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_dynamic_constant_assignment(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_self(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_nil(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_true(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_false(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_file(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_line(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_encoding(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_assign_to_numparam(
        level: ErrorLevel,
        loc: *mut Loc,
        numparam: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_cant_set_variable(
        level: ErrorLevel,
        loc: *mut Loc,
        var_name: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_block_given_to_yield(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_block_and_block_arg_given(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_symbol_literal_with_interpolation(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_reserved_for_numparam(
        level: ErrorLevel,
        loc: *mut Loc,
        numparam: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_key_must_be_valid_as_local_variable(
        level: ErrorLevel,
        loc: *mut Loc,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_duplicate_variable_name(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_duplicate_key_name(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_singleton_literal(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_nth_ref_is_too_big(
        level: ErrorLevel,
        loc: *mut Loc,
        nth_ref: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_duplicated_argument_name(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_regex_error(level: ErrorLevel, loc: *mut Loc, error: BytePtr) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_invalid_symbol(
        level: ErrorLevel,
        loc: *mut Loc,
        symbol: BytePtr,
    ) -> *mut Diagnostic;
}
extern "C" {
    pub fn make_void_value_expression(level: ErrorLevel, loc: *mut Loc) -> *mut Diagnostic;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommentType {
    INLINE = 0,
    DOCUMENT = 1,
    UNKNOWN = 2,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MagicCommentKind {
    ENCODING = 0,
    FROZEN_STRING_LITERAL = 1,
    WARN_INDENT = 2,
    SHAREABLE_CONSTANT_VALUE = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Token {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Comment {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CustomDecoder {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MagicComment {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TokenRewriter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParserResult {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParserOptions {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct TokenVec {
    pub ptr: *mut *mut Token,
    pub length: u32,
}
#[repr(C)]
pub struct DiagnosticVec {
    pub ptr: *mut *mut Diagnostic,
    pub length: u32,
}
#[repr(C)]
pub struct CommentVec {
    pub ptr: *mut *mut Comment,
    pub length: u32,
}
#[repr(C)]
pub struct MagicCommentVec {
    pub ptr: *mut *mut MagicComment,
    pub length: u32,
}
extern "C" {
    pub fn make_parser_result(
        ast: *mut Node,
        tokens: TokenVec,
        diagnostics: DiagnosticVec,
        comments: CommentVec,
        magic_comments: MagicCommentVec,
        input: *mut ::std::os::raw::c_void,
    ) -> *mut ParserResult;
}
extern "C" {
    pub fn make_comment(kind: CommentType, location: *mut Loc) -> *mut Comment;
}
extern "C" {
    pub fn make_magic_comment(
        kind: MagicCommentKind,
        key_l: *mut Loc,
        value_l: *mut Loc,
    ) -> *mut MagicComment;
}
extern "C" {
    pub fn make_loc(begin: u32, end: u32) -> *mut Loc;
}
extern "C" {
    pub fn make_token(
        token_type: ::std::os::raw::c_int,
        token_value: BytePtr,
        loc: *mut Loc,
    ) -> *mut Token;
}
#[repr(C)]
pub struct CustomDecoderResult {
    pub success: bool,
    pub output: BytePtr,
    pub error_message: BytePtr,
}
extern "C" {
    pub fn rewrite(
        decoder: *mut CustomDecoder,
        encoding: BytePtr,
        input: BytePtr,
    ) -> CustomDecoderResult;
}
extern "C" {
    pub fn parser_options_buffer_name(options: *mut ParserOptions) -> BytePtr;
}
extern "C" {
    pub fn parser_options_custom_decoder(options: *mut ParserOptions) -> *mut CustomDecoder;
}
extern "C" {
    pub fn parser_options_token_rewriter(options: *mut ParserOptions) -> *mut TokenRewriter;
}
extern "C" {
    pub fn parser_options_debug(options: *mut ParserOptions) -> bool;
}
extern "C" {
    pub fn parser_options_record_tokens(options: *mut ParserOptions) -> bool;
}
extern "C" {
    pub fn free_str(s: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn free_byte_ptr2(byte_ptr: BytePtr);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RawRewriteAction {
    REWRITE_ACTION_KEEP = 0,
    REWRITE_ACTION_DROP = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RawLexStateAction {
    LEX_STATE_SET = 0,
    LEX_STATE_KEEP = 1,
}
#[repr(C)]
pub struct RawToken {
    pub token_type: ::std::os::raw::c_int,
    pub token_value: BytePtr,
    pub loc_begin: u32,
    pub loc_end: u32,
}
#[repr(C)]
pub struct RawTokenRewriterResult {
    pub rewrite_action: RawRewriteAction,
    pub lex_state_action: RawLexStateAction,
    pub next_state: ::std::os::raw::c_int,
    pub token: RawToken,
}
extern "C" {
    pub fn rewrite_token(
        rewriter: *mut TokenRewriter,
        token: RawToken,
        input: BytePtr,
    ) -> RawTokenRewriterResult;
}
extern "C" {
    pub fn loc_begin(loc: *mut Loc) -> u32;
}
extern "C" {
    pub fn loc_end(loc: *mut Loc) -> u32;
}
