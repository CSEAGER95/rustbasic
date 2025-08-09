mod todo;
mod constants;
mod mathematicaloperators;
mod boolean;
mod chartype;
mod tuples;
mod arrays;
mod functions;
mod conditionalstatements;
mod loops;
mod stringtype;
mod referencing;
mod slicetype;
mod enumerationanditerators;
mod structs;
mod methodsandimplementations;
mod enums;
mod matchkeyword;

fn main() {
    todo::run_todo();
    constants::run_constants();
    mathematicaloperators::run_mathematicaloperators();
    boolean::run_boolean();
    chartype::run_chartype();
    tuples::run_tuples();
    arrays::run_arrays();
    functions::run_functions();
    conditionalstatements::run_conditionalstatements();
    loops::run_loops();
    stringtype::run_stringtype();
    referencing::run_referencing();
    slicetype::run_slicetype();
    enumerationanditerators::run_enumerationanditerators();
    structs::run_structs();
    methodsandimplementations::run_methodsandimplementations();
    enums::run_enums();
    matchkeyword::run_matchkeyword();
}
