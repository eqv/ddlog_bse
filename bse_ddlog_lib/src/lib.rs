// The main auto-generated crate `<progname>_ddlog` (`tutorial_ddlog`
// in this case) declares `HDDlog` type that serves as a reference to a
// running DDlog program.
// `HDDlog` implements `trait differential_datalog::DDlog` (see below).
use bse_ddlog::api::HDDlog;

// The differential_datalog crate contains the DDlog runtime that is
// the same for all DDlog programs and simply gets copied to each generated
// DDlog workspace unmodified (this will change in future releases).
use differential_datalog::DDlog; // Trait that must be implemented by an instance of a DDlog program.
use differential_datalog::DeltaMap; // Type that represents a set of changes to DDlog relations.
                                    // Returned by `DDlog::transaction_commit_dump_changes()`.
use differential_datalog::ddval::DDValue; // Generic type that wraps all DDlog value.
use differential_datalog::ddval::DDValConvert; // Trait to convert Rust types to/from DDValue.
                                               // All types in the `value::Value` module (see below)
                                               // implement this trait.
use differential_datalog::program::RelId; // Numeric relations id.
use differential_datalog::program::Update; // Type-safe representation of a DDlog command (insert/delete_val/delete_key/...)

// The `record` module defines dynamically typed representation of DDlog values and commands.
use differential_datalog::record::Record; // Dynamically typed representation of DDlog values.

// The auto-generated `types` crate contains Rust types that correspond to user-defined DDlog
// types, one for each typedef and each relation in the DDlog program.
use types::*;
use types::ddlog_std::Ref;

// The auto-generated `value` crate contains
// * The `Value` model that define a wrapper type for each input and
//   output relation in the DDlog program, as well as
// * `enum Relations` that enumerates program relations
// * Several functions that convert between numeric relation id's and
//   symbolic names.
use value::relid2name;
use value::Relations;
use value::Value;

pub struct DDLogBSE{
    hddlog: HDDlog,
}
impl DDLogBSE{
    pub fn new()  -> Result<DDLogBSE, String> {
        fn cb(_rel: usize, _rec: &Record, _w: isize) {}

        // Instantiate a DDlog program.
        // Returns a handle to the program and initial contents of output relations.
        // Arguments
        // - number of worker threads (you typically want 1 or 2).
        // - Boolean flag that indicates whether DDlog will track the complete snapshot
        //   of output relations.  Should only be used if you plan to dump `dump_table`
        //   their contents using `HDDlog::dump_table()`.
        // - Callback - obsolete and will disappear in future releases.
        let (hddlog, _init_state) = HDDlog::run(1, false, cb)?;
        return Ok(Self{hddlog});
    }

    pub fn add_code(&mut self, code: Vec<InstrT>) -> Result<DeltaMap<DDValue>, String>{
        self.hddlog.transaction_start()?;
        let updates = code.into_iter().enumerate().map(|(i,inst)| 
            Update::Insert {
                relid: Relations::Instr as RelId,
                v: Value::Instr(types::Instr{i: i as u32, op: inst}).into_ddvalue(),
            }
        ).collect::<Vec<_>>();
        self.hddlog.apply_valupdates(updates.into_iter())?;   
        let delta = self.hddlog.transaction_commit_dump_changes()?;
        return Ok(delta); 
    }

    pub fn dump_delta(delta: &DeltaMap<DDValue>) {
        for (rel, changes) in delta.iter() {
            println!("Changes to relation {}", relid2name(*rel).unwrap());
            for (val, weight) in changes.iter() {
                println!("{} {:+}", val, weight);
            }
        }
    }

    pub fn enum_events(delta: &mut DeltaMap<DDValue>) {

        // Retrieve the set of changes for a particular relation.
        let new_phrases = delta.get_rel(Relations::EventOn as RelId);
        for (val, weight) in new_phrases.iter() {
            // weight = 1 - insert.
            // weight = -1 - delete.
            assert_eq!(*weight, 1);
            // `val` has type `DDValue`; converting it to a concrete Rust
            // type is an unsafe operation: specifying the wrong Rust type
            // will lead to undefined behavior.
            let event: &Value::EventOn = unsafe { Value::EventOn::from_ddvalue_ref(val) };
            println!("New event: {:?}", event.0);
        };
    }

    pub fn stop(&mut self){
        self.hddlog.stop().unwrap();
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let malloc = InstrT::Malloc{arg: Expr64T::Const64{v: 10} };
        let cmp = InstrT::Cmp{cop1: Expr64T::Reg64{r: Reg64T::Rbx}, cop2: Expr64T::Const64{v: 128}};
        let jmp = InstrT::Jump{target: 6, cond: Expr1T::Reg1{r: Reg1T::FlagGEq}};
        let set = InstrT::Set{dst: Reg64T::Rax,src:
             Expr64T::BinOp64{op: Op64T::Add,
                a1: Ref::from(Expr64T::Reg64{r: Reg64T::Rax}), 
                a2: Ref::from(Expr64T::Reg64{r: Reg64T::Rbx})
            }
        };
        let store = InstrT::MovPtr{ptr: Expr64T::Reg64{r: Reg64T::Rax},a2: Expr64T::Const64{v: 0}};
        
        let mut bse = DDLogBSE::new().unwrap();
        let mut delta = bse.add_code(vec!(malloc, cmp, jmp, set, store)).unwrap();
        //DDLogBSE::dump_delta(&delta);
        DDLogBSE::enum_events(&mut delta);
        assert!(false);
    }
}
