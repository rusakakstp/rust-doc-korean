// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Detecting language items.
//
// Language items are items that represent concepts intrinsic to the language
// itself. Examples are:
//
// * Traits that specify "kinds"; e.g. "Freeze", "Copy", "Send".
//
// * Traits that represent operators; e.g. "Add", "Sub", "Index".
//
// * Functions called by the compiler itself.


use driver::session::Session;
use metadata::csearch::each_lang_item;
use metadata::cstore::iter_crate_data;
use syntax::ast::{crate, def_id, lit_str, meta_item};
use syntax::ast::{meta_list, meta_name_value, meta_word};
use syntax::ast_util::local_def;
use syntax::visit::{default_simple_visitor, mk_simple_visitor, SimpleVisitor};
use syntax::visit::visit_crate;

use std::hashmap::HashMap;

pub enum LangItem {
    FreezeTraitLangItem,               // 0
    CopyTraitLangItem,                 // 1
    SendTraitLangItem,                 // 2
    SizedTraitLangItem,                // 3

    DropTraitLangItem,                 // 4

    AddTraitLangItem,                  // 5
    SubTraitLangItem,                  // 6
    MulTraitLangItem,                  // 7
    DivTraitLangItem,                  // 8
    RemTraitLangItem,                  // 9
    NegTraitLangItem,                  // 10
    NotTraitLangItem,                  // 11
    BitXorTraitLangItem,               // 11
    BitAndTraitLangItem,               // 13
    BitOrTraitLangItem,                // 14
    ShlTraitLangItem,                  // 15
    ShrTraitLangItem,                  // 16
    IndexTraitLangItem,                // 17

    EqTraitLangItem,                   // 18
    OrdTraitLangItem,                  // 19

    StrEqFnLangItem,                   // 20
    UniqStrEqFnLangItem,               // 21
    AnnihilateFnLangItem,              // 22
    LogTypeFnLangItem,                 // 23
    FailFnLangItem,                    // 24
    FailBoundsCheckFnLangItem,         // 25
    ExchangeMallocFnLangItem,          // 26
    VectorExchangeMallocFnLangItem,    // 27
    ClosureExchangeMallocFnLangItem,   // 28
    ExchangeFreeFnLangItem,            // 29
    MallocFnLangItem,                  // 30
    FreeFnLangItem,                    // 31
    BorrowAsImmFnLangItem,             // 32
    BorrowAsMutFnLangItem,             // 33
    ReturnToMutFnLangItem,             // 34
    CheckNotBorrowedFnLangItem,        // 35
    StrDupUniqFnLangItem,              // 36
    RecordBorrowFnLangItem,            // 37
    UnrecordBorrowFnLangItem,          // 38

    StartFnLangItem,                   // 39

    TyDescStructLangItem,              // 40
    TyVisitorTraitLangItem,            // 41
    OpaqueStructLangItem,              // 42
}

pub struct LanguageItems {
    items: [Option<def_id>, ..43]
}

impl LanguageItems {
    pub fn new() -> LanguageItems {
        LanguageItems {
            items: [ None, ..43 ]
        }
    }

    pub fn each_item(&self, f: &fn(def_id: def_id, i: uint) -> bool) -> bool {
        self.items.iter().enumerate().advance(|(i, &item)| f(item.get(), i))
    }

    pub fn item_name(index: uint) -> &'static str {
        match index {
            0  => "freeze",
            1  => "copy",
            2  => "send",
            3  => "sized",

            4  => "drop",

            5  => "add",
            6  => "sub",
            7  => "mul",
            8  => "div",
            9  => "rem",
            10 => "neg",
            11 => "not",
            12 => "bitxor",
            13 => "bitand",
            14 => "bitor",
            15 => "shl",
            16 => "shr",
            17 => "index",
            18 => "eq",
            19 => "ord",

            20 => "str_eq",
            21 => "uniq_str_eq",
            22 => "annihilate",
            23 => "log_type",
            24 => "fail_",
            25 => "fail_bounds_check",
            26 => "exchange_malloc",
            27 => "vector_exchange_malloc",
            28 => "closure_exchange_malloc",
            29 => "exchange_free",
            30 => "malloc",
            31 => "free",
            32 => "borrow_as_imm",
            33 => "borrow_as_mut",
            34 => "return_to_mut",
            35 => "check_not_borrowed",
            36 => "strdup_uniq",
            37 => "record_borrow",
            38 => "unrecord_borrow",

            39 => "start",

            40 => "ty_desc",
            41 => "ty_visitor",
            42 => "opaque",

            _ => "???"
        }
    }

    // FIXME #4621: Method macros sure would be nice here.

    pub fn freeze_trait(&self) -> def_id {
        self.items[FreezeTraitLangItem as uint].get()
    }
    pub fn copy_trait(&self) -> def_id {
        self.items[CopyTraitLangItem as uint].get()
    }
    pub fn send_trait(&self) -> def_id {
        self.items[SendTraitLangItem as uint].get()
    }
    pub fn sized_trait(&self) -> def_id {
        self.items[SizedTraitLangItem as uint].get()
    }

    pub fn drop_trait(&self) -> def_id {
        self.items[DropTraitLangItem as uint].get()
    }

    pub fn add_trait(&self) -> def_id {
        self.items[AddTraitLangItem as uint].get()
    }
    pub fn sub_trait(&self) -> def_id {
        self.items[SubTraitLangItem as uint].get()
    }
    pub fn mul_trait(&self) -> def_id {
        self.items[MulTraitLangItem as uint].get()
    }
    pub fn div_trait(&self) -> def_id {
        self.items[DivTraitLangItem as uint].get()
    }
    pub fn rem_trait(&self) -> def_id {
        self.items[RemTraitLangItem as uint].get()
    }
    pub fn neg_trait(&self) -> def_id {
        self.items[NegTraitLangItem as uint].get()
    }
    pub fn not_trait(&self) -> def_id {
        self.items[NotTraitLangItem as uint].get()
    }
    pub fn bitxor_trait(&self) -> def_id {
        self.items[BitXorTraitLangItem as uint].get()
    }
    pub fn bitand_trait(&self) -> def_id {
        self.items[BitAndTraitLangItem as uint].get()
    }
    pub fn bitor_trait(&self) -> def_id {
        self.items[BitOrTraitLangItem as uint].get()
    }
    pub fn shl_trait(&self) -> def_id {
        self.items[ShlTraitLangItem as uint].get()
    }
    pub fn shr_trait(&self) -> def_id {
        self.items[ShrTraitLangItem as uint].get()
    }
    pub fn index_trait(&self) -> def_id {
        self.items[IndexTraitLangItem as uint].get()
    }

    pub fn eq_trait(&self) -> def_id {
        self.items[EqTraitLangItem as uint].get()
    }
    pub fn ord_trait(&self) -> def_id {
        self.items[OrdTraitLangItem as uint].get()
    }

    pub fn str_eq_fn(&self) -> def_id {
        self.items[StrEqFnLangItem as uint].get()
    }
    pub fn uniq_str_eq_fn(&self) -> def_id {
        self.items[UniqStrEqFnLangItem as uint].get()
    }
    pub fn annihilate_fn(&self) -> def_id {
        self.items[AnnihilateFnLangItem as uint].get()
    }
    pub fn log_type_fn(&self) -> def_id {
        self.items[LogTypeFnLangItem as uint].get()
    }
    pub fn fail_fn(&self) -> def_id {
        self.items[FailFnLangItem as uint].get()
    }
    pub fn fail_bounds_check_fn(&self) -> def_id {
        self.items[FailBoundsCheckFnLangItem as uint].get()
    }
    pub fn exchange_malloc_fn(&self) -> def_id {
        self.items[ExchangeMallocFnLangItem as uint].get()
    }
    pub fn vector_exchange_malloc_fn(&self) -> def_id {
        self.items[VectorExchangeMallocFnLangItem as uint].get()
    }
    pub fn closure_exchange_malloc_fn(&self) -> def_id {
        self.items[ClosureExchangeMallocFnLangItem as uint].get()
    }
    pub fn exchange_free_fn(&self) -> def_id {
        self.items[ExchangeFreeFnLangItem as uint].get()
    }
    pub fn malloc_fn(&self) -> def_id {
        self.items[MallocFnLangItem as uint].get()
    }
    pub fn free_fn(&self) -> def_id {
        self.items[FreeFnLangItem as uint].get()
    }
    pub fn borrow_as_imm_fn(&self) -> def_id {
        self.items[BorrowAsImmFnLangItem as uint].get()
    }
    pub fn borrow_as_mut_fn(&self) -> def_id {
        self.items[BorrowAsMutFnLangItem as uint].get()
    }
    pub fn return_to_mut_fn(&self) -> def_id {
        self.items[ReturnToMutFnLangItem as uint].get()
    }
    pub fn check_not_borrowed_fn(&self) -> def_id {
        self.items[CheckNotBorrowedFnLangItem as uint].get()
    }
    pub fn strdup_uniq_fn(&self) -> def_id {
        self.items[StrDupUniqFnLangItem as uint].get()
    }
    pub fn record_borrow_fn(&self) -> def_id {
        self.items[RecordBorrowFnLangItem as uint].get()
    }
    pub fn unrecord_borrow_fn(&self) -> def_id {
        self.items[UnrecordBorrowFnLangItem as uint].get()
    }
    pub fn start_fn(&self) -> def_id {
        self.items[StartFnLangItem as uint].get()
    }
    pub fn ty_desc(&const self) -> def_id {
        self.items[TyDescStructLangItem as uint].get()
    }
    pub fn ty_visitor(&const self) -> def_id {
        self.items[TyVisitorTraitLangItem as uint].get()
    }
    pub fn opaque(&const self) -> def_id {
        self.items[OpaqueStructLangItem as uint].get()
    }
}

struct LanguageItemCollector<'self> {
    items: LanguageItems,

    crate: &'self crate,
    session: Session,

    item_refs: HashMap<@str, uint>,
}

impl<'self> LanguageItemCollector<'self> {
    pub fn new<'a>(crate: &'a crate, session: Session)
                   -> LanguageItemCollector<'a> {
        let mut item_refs = HashMap::new();

        item_refs.insert(@"freeze", FreezeTraitLangItem as uint);
        item_refs.insert(@"copy", CopyTraitLangItem as uint);
        item_refs.insert(@"send", SendTraitLangItem as uint);
        item_refs.insert(@"sized", SizedTraitLangItem as uint);

        item_refs.insert(@"drop", DropTraitLangItem as uint);

        item_refs.insert(@"add", AddTraitLangItem as uint);
        item_refs.insert(@"sub", SubTraitLangItem as uint);
        item_refs.insert(@"mul", MulTraitLangItem as uint);
        item_refs.insert(@"div", DivTraitLangItem as uint);
        item_refs.insert(@"rem", RemTraitLangItem as uint);
        item_refs.insert(@"neg", NegTraitLangItem as uint);
        item_refs.insert(@"not", NotTraitLangItem as uint);
        item_refs.insert(@"bitxor", BitXorTraitLangItem as uint);
        item_refs.insert(@"bitand", BitAndTraitLangItem as uint);
        item_refs.insert(@"bitor", BitOrTraitLangItem as uint);
        item_refs.insert(@"shl", ShlTraitLangItem as uint);
        item_refs.insert(@"shr", ShrTraitLangItem as uint);
        item_refs.insert(@"index", IndexTraitLangItem as uint);

        item_refs.insert(@"eq", EqTraitLangItem as uint);
        item_refs.insert(@"ord", OrdTraitLangItem as uint);

        item_refs.insert(@"str_eq", StrEqFnLangItem as uint);
        item_refs.insert(@"uniq_str_eq", UniqStrEqFnLangItem as uint);
        item_refs.insert(@"annihilate", AnnihilateFnLangItem as uint);
        item_refs.insert(@"log_type", LogTypeFnLangItem as uint);
        item_refs.insert(@"fail_", FailFnLangItem as uint);
        item_refs.insert(@"fail_bounds_check",
                         FailBoundsCheckFnLangItem as uint);
        item_refs.insert(@"exchange_malloc", ExchangeMallocFnLangItem as uint);
        item_refs.insert(@"vector_exchange_malloc", VectorExchangeMallocFnLangItem as uint);
        item_refs.insert(@"closure_exchange_malloc", ClosureExchangeMallocFnLangItem as uint);
        item_refs.insert(@"exchange_free", ExchangeFreeFnLangItem as uint);
        item_refs.insert(@"malloc", MallocFnLangItem as uint);
        item_refs.insert(@"free", FreeFnLangItem as uint);
        item_refs.insert(@"borrow_as_imm", BorrowAsImmFnLangItem as uint);
        item_refs.insert(@"borrow_as_mut", BorrowAsMutFnLangItem as uint);
        item_refs.insert(@"return_to_mut", ReturnToMutFnLangItem as uint);
        item_refs.insert(@"check_not_borrowed",
                         CheckNotBorrowedFnLangItem as uint);
        item_refs.insert(@"strdup_uniq", StrDupUniqFnLangItem as uint);
        item_refs.insert(@"record_borrow", RecordBorrowFnLangItem as uint);
        item_refs.insert(@"unrecord_borrow", UnrecordBorrowFnLangItem as uint);
        item_refs.insert(@"start", StartFnLangItem as uint);
        item_refs.insert(@"ty_desc", TyDescStructLangItem as uint);
        item_refs.insert(@"ty_visitor", TyVisitorTraitLangItem as uint);
        item_refs.insert(@"opaque", OpaqueStructLangItem as uint);

        LanguageItemCollector {
            crate: crate,
            session: session,
            items: LanguageItems::new(),
            item_refs: item_refs
        }
    }

    pub fn match_and_collect_meta_item(&mut self,
                                       item_def_id: def_id,
                                       meta_item: &meta_item) {
        match meta_item.node {
            meta_name_value(key, literal) => {
                match literal.node {
                    lit_str(value) => {
                        self.match_and_collect_item(item_def_id, key, value);
                    }
                    _ => {} // Skip.
                }
            }
            meta_word(*) | meta_list(*) => {} // Skip.
        }
    }

    pub fn collect_item(&mut self, item_index: uint, item_def_id: def_id) {
        // Check for duplicates.
        match self.items.items[item_index] {
            Some(original_def_id) if original_def_id != item_def_id => {
                self.session.err(fmt!("duplicate entry for `%s`",
                                      LanguageItems::item_name(item_index)));
            }
            Some(_) | None => {
                // OK.
            }
        }

        // Matched.
        self.items.items[item_index] = Some(item_def_id);
    }

    pub fn match_and_collect_item(&mut self,
                                  item_def_id: def_id,
                                  key: &str,
                                  value: @str) {
        if "lang" != key {
            return;    // Didn't match.
        }

        let item_index = self.item_refs.find(&value).map(|x| **x);
        // prevent borrow checker from considering   ^~~~~~~~~~~
        // self to be borrowed (annoying)

        match item_index {
            Some(item_index) => {
                self.collect_item(item_index, item_def_id);
            }
            None => {
                // Didn't match.
                return;
            }
        }
    }

    pub fn collect_local_language_items(&mut self) {
        let this: *mut LanguageItemCollector = &mut *self;
        visit_crate(self.crate, ((), mk_simple_visitor(@SimpleVisitor {
            visit_item: |item| {
                for item.attrs.iter().advance |attribute| {
                    unsafe {
                        (*this).match_and_collect_meta_item(
                            local_def(item.id),
                            attribute.node.value
                        );
                    }
                }
            },
            .. *default_simple_visitor()
        })));
    }

    pub fn collect_external_language_items(&mut self) {
        let crate_store = self.session.cstore;
        do iter_crate_data(crate_store) |crate_number, _crate_metadata| {
            for each_lang_item(crate_store, crate_number)
                    |node_id, item_index| {
                let def_id = def_id { crate: crate_number, node: node_id };
                self.collect_item(item_index, def_id);
            }
        }
    }

    pub fn check_completeness(&self) {
        for self.item_refs.iter().advance |(&key, &item_ref)| {
            match self.items.items[item_ref] {
                None => {
                    self.session.err(fmt!("no item found for `%s`", key));
                }
                Some(_) => {
                    // OK.
                }
            }
        }
    }

    pub fn collect(&mut self) {
        self.collect_local_language_items();
        self.collect_external_language_items();
        self.check_completeness();
    }
}

pub fn collect_language_items(crate: &crate,
                              session: Session)
                           -> LanguageItems {
    let mut collector = LanguageItemCollector::new(crate, session);
    collector.collect();
    let LanguageItemCollector { items, _ } = collector;
    session.abort_if_errors();
    items
}
