/*
namespace: Parse
expectation: Pass
*/

circuit X {
    static const a: u8 = 10;
    x: u32,
    y: u32
    function x() {
        return ();
    }
    function y() {
        return ();
    }
}