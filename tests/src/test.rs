use obj2str::Obj2Str;
use obj2str_derive::Obj2Str;

#[derive(Clone, Obj2Str)]
struct TypeTest {
    f01: u8,
    f02: u16,
    f03: u32,
    f04: u64,
    f05: u128,
    f06: usize,
    f07: i8,
    f08: i16,
    f09: i32,
    f10: i64,
    f11: i128,
    f12: isize,
    f13: f32,
    f14: f64,
    f15: bool,
    f16: char,
    f17: String,
    f18: (),
    f19: (u8,),
    f20: (u8, u16),
    f21: (u8, u16, u32),
    f22: (u8, u16, u32, u64),
    f23: Option<u8>,
    f24: Option<u8>,
    f25: Result<u8, String>,
    f26: Result<u8, String>,
    f27: Vec<u16>,
    f28: Vec<()>,
}

impl Default for TypeTest {
    fn default() -> Self {
        TypeTest {
            f01: 8,
            f02: 16,
            f03: 32,
            f04: 64,
            f05: 128,
            f06: 256,
            f07: -8,
            f08: -16,
            f09: -32,
            f10: -64,
            f11: -128,
            f12: -256,
            f13: 3.5,
            f14: 9.6,
            f15: true,
            f16: 'o',
            f17: "obj2str".to_string(),
            f18: (),
            f19: (8,),
            f20: (8, 16),
            f21: (8, 16, 32),
            f22: (8, 16, 32, 64),
            f23: Some(42),
            f24: None,
            f25: Ok(100),
            f26: Err("001".to_string()),
            f27: vec![0, 1, 2, 3],
            f28: vec![],
        }
    }
}

#[derive(Obj2Str)]
struct AggregateTest {
    f01: u8,
    f02: Vec<u8>,
    f03: TypeTest,
    f04: Vec<TypeTest>,
}

impl Default for AggregateTest {
    fn default() -> Self {
        AggregateTest {
            f01: 2,
            f02: vec![0, 1],
            f03: TypeTest::default(),
            f04: vec![TypeTest::default(); 3],
        }
    }
}

#[derive(Obj2Str)]
struct FieldWidthTest {
    first: u8,
    bigger: u8,
    the_biggest: u8,
    smaller: u8,
}

impl Default for FieldWidthTest {
    fn default() -> Self {
        FieldWidthTest {
            first: 0,
            bigger: 0,
            the_biggest: 0,
            smaller: 0,
        }
    }
}

#[derive(Obj2Str)]
struct IndexWidthTest {
    collection: Vec<usize>,
}

#[test]
fn type_test_01() {
    assert!(TypeTest::default().obj2str(0, 1) == "TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }");
}

#[test]
fn aggregate_test_01() {
    assert!(
        AggregateTest::default().obj2str(0, 1)
            == "AggregateTest { f01: 2, f02: [...] (2), f03: TypeTest {...}, f04: [...] (3) }"
    );
}

#[test]
fn aggregate_test_02() {
    assert!(
        AggregateTest::default().obj2str(1, 1)
            == "AggregateTest {
	f01: 2,
	f02: [...] (2),
	f03: TypeTest {...},
	f04: [...] (3),
}"
    );
}

#[test]
fn aggregate_test_03() {
    assert!(AggregateTest::default().obj2str(0, 2) == "AggregateTest { f01: 2, f02: [ 0, 1 ], f03: TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }, f04: [ TypeTest {...}, TypeTest {...}, TypeTest {...} ] }");
}

#[test]
fn aggregate_test_04() {
    assert!(
        AggregateTest::default().obj2str(1, 2)
            == "AggregateTest {
	f01: 2,
	f02: [
		(0) 0,
		(1) 1,
	],
	f03: TypeTest {
		f01: 8,
		f02: 16,
		f03: 32,
		f04: 64,
		f05: 128,
		f06: 256,
		f07: -8,
		f08: -16,
		f09: -32,
		f10: -64,
		f11: -128,
		f12: -256,
		f13: 3.5000000,
		f14: 9.600000000000000,
		f15: true,
		f16: 'o',
		f17: \"obj2str\",
		f18: (),
		f19: (
			8,
		),
		f20: (
			8,
			16,
		),
		f21: (
			8,
			16,
			32,
		),
		f22: (
			8,
			16,
			32,
			64,
		),
		f23: Some,
		f24: None,
		f25: Ok,
		f26: Err,
		f27: [...] (4),
		f28: [...] (0),
	},
	f04: [
		(0) TypeTest {...},
		(1) TypeTest {...},
		(2) TypeTest {...},
	],
}"
    );
}

#[test]
fn verbosity_test_01() {
    assert!(AggregateTest::default().obj2str(0, -1) == "AggregateTest {...}");
}

#[test]
fn verbosity_test_02() {
    assert!(AggregateTest::default().obj2str(0, 0) == "AggregateTest {...}");
}

#[test]
fn verbosity_test_03() {
    assert!(
        AggregateTest::default().obj2str(0, 1)
            == "AggregateTest { f01: 2, f02: [...] (2), f03: TypeTest {...}, f04: [...] (3) }"
    );
}

#[test]
fn verbosity_test_04() {
    assert!(AggregateTest::default().obj2str(0, 2) == "AggregateTest { f01: 2, f02: [ 0, 1 ], f03: TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }, f04: [ TypeTest {...}, TypeTest {...}, TypeTest {...} ] }");
}

#[test]
fn verbosity_test_05() {
    assert!(AggregateTest::default().obj2str(0, 3) == "AggregateTest { f01: 2, f02: [ 0, 1 ], f03: TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some(42), f24: None, f25: Ok(100), f26: Err(\"001\"), f27: [ 0, 1, 2, 3 ], f28: [] }, f04: [ TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }, TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }, TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) } ] }");
}

#[test]
fn indentation_test_01() {
    assert!(AggregateTest::default().obj2str(-1, 2) == "AggregateTest { f01: 2, f02: [ 0, 1 ], f03: TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }, f04: [ TypeTest {...}, TypeTest {...}, TypeTest {...} ] }");
}

#[test]
fn indentation_test_02() {
    assert!(AggregateTest::default().obj2str(0, 2) == "AggregateTest { f01: 2, f02: [ 0, 1 ], f03: TypeTest { f01: 8, f02: 16, f03: 32, f04: 64, f05: 128, f06: 256, f07: -8, f08: -16, f09: -32, f10: -64, f11: -128, f12: -256, f13: 3.5000000, f14: 9.600000000000000, f15: true, f16: 'o', f17: \"obj2str\", f18: (), f19: ( 8 ), f20: ( 8, 16 ), f21: ( 8, 16, 32 ), f22: ( 8, 16, 32, 64 ), f23: Some, f24: None, f25: Ok, f26: Err, f27: [...] (4), f28: [...] (0) }, f04: [ TypeTest {...}, TypeTest {...}, TypeTest {...} ] }");
}

#[test]
fn indentation_test_03() {
    assert!(
        AggregateTest::default().obj2str(1, 2)
            == "AggregateTest {
	f01: 2,
	f02: [
		(0) 0,
		(1) 1,
	],
	f03: TypeTest {
		f01: 8,
		f02: 16,
		f03: 32,
		f04: 64,
		f05: 128,
		f06: 256,
		f07: -8,
		f08: -16,
		f09: -32,
		f10: -64,
		f11: -128,
		f12: -256,
		f13: 3.5000000,
		f14: 9.600000000000000,
		f15: true,
		f16: 'o',
		f17: \"obj2str\",
		f18: (),
		f19: (
			8,
		),
		f20: (
			8,
			16,
		),
		f21: (
			8,
			16,
			32,
		),
		f22: (
			8,
			16,
			32,
			64,
		),
		f23: Some,
		f24: None,
		f25: Ok,
		f26: Err,
		f27: [...] (4),
		f28: [...] (0),
	},
	f04: [
		(0) TypeTest {...},
		(1) TypeTest {...},
		(2) TypeTest {...},
	],
}"
    );
}

#[test]
fn indentation_test_04() {
    assert!(
        AggregateTest::default().obj2str(2, 2)
            == "AggregateTest {
		f01: 2,
		f02: [
			(0) 0,
			(1) 1,
		],
		f03: TypeTest {
			f01: 8,
			f02: 16,
			f03: 32,
			f04: 64,
			f05: 128,
			f06: 256,
			f07: -8,
			f08: -16,
			f09: -32,
			f10: -64,
			f11: -128,
			f12: -256,
			f13: 3.5000000,
			f14: 9.600000000000000,
			f15: true,
			f16: 'o',
			f17: \"obj2str\",
			f18: (),
			f19: (
				8,
			),
			f20: (
				8,
				16,
			),
			f21: (
				8,
				16,
				32,
			),
			f22: (
				8,
				16,
				32,
				64,
			),
			f23: Some,
			f24: None,
			f25: Ok,
			f26: Err,
			f27: [...] (4),
			f28: [...] (0),
		},
		f04: [
			(0) TypeTest {...},
			(1) TypeTest {...},
			(2) TypeTest {...},
		],
	}"
    );
}

#[test]
fn field_width_test_01() {
    assert!(
        FieldWidthTest::default().obj2str(1, 1)
            == "FieldWidthTest {
	first:       0,
	bigger:      0,
	the_biggest: 0,
	smaller:     0,
}"
    );
}

#[test]
fn index_width_test_01() {
    let test_struct = IndexWidthTest {
        collection: vec![0; 0],
    };
    assert!(
        test_struct.obj2str(1, 2)
            == "IndexWidthTest {
	collection: [],
}"
    );
}

#[test]
fn index_width_test_02() {
    let test_struct = IndexWidthTest {
        collection: vec![0; 10],
    };
    assert!(
        test_struct.obj2str(1, 2)
            == "IndexWidthTest {
	collection: [
		(0) 0,
		(1) 0,
		(2) 0,
		(3) 0,
		(4) 0,
		(5) 0,
		(6) 0,
		(7) 0,
		(8) 0,
		(9) 0,
	],
}"
    );
}

#[test]
fn index_width_test_03() {
    let test_struct = IndexWidthTest {
        collection: vec![0; 11],
    };
    assert!(
        test_struct.obj2str(1, 2)
            == "IndexWidthTest {
	collection: [
		( 0) 0,
		( 1) 0,
		( 2) 0,
		( 3) 0,
		( 4) 0,
		( 5) 0,
		( 6) 0,
		( 7) 0,
		( 8) 0,
		( 9) 0,
		(10) 0,
	],
}"
    );
}

#[test]
fn index_width_test_04() {
    let test_struct = IndexWidthTest {
        collection: vec![0; 100],
    };
    assert!(
        test_struct.obj2str(1, 2)
            == "IndexWidthTest {
	collection: [
		( 0) 0,
		( 1) 0,
		( 2) 0,
		( 3) 0,
		( 4) 0,
		( 5) 0,
		( 6) 0,
		( 7) 0,
		( 8) 0,
		( 9) 0,
		(10) 0,
		(11) 0,
		(12) 0,
		(13) 0,
		(14) 0,
		(15) 0,
		(16) 0,
		(17) 0,
		(18) 0,
		(19) 0,
		(20) 0,
		(21) 0,
		(22) 0,
		(23) 0,
		(24) 0,
		(25) 0,
		(26) 0,
		(27) 0,
		(28) 0,
		(29) 0,
		(30) 0,
		(31) 0,
		(32) 0,
		(33) 0,
		(34) 0,
		(35) 0,
		(36) 0,
		(37) 0,
		(38) 0,
		(39) 0,
		(40) 0,
		(41) 0,
		(42) 0,
		(43) 0,
		(44) 0,
		(45) 0,
		(46) 0,
		(47) 0,
		(48) 0,
		(49) 0,
		(50) 0,
		(51) 0,
		(52) 0,
		(53) 0,
		(54) 0,
		(55) 0,
		(56) 0,
		(57) 0,
		(58) 0,
		(59) 0,
		(60) 0,
		(61) 0,
		(62) 0,
		(63) 0,
		(64) 0,
		(65) 0,
		(66) 0,
		(67) 0,
		(68) 0,
		(69) 0,
		(70) 0,
		(71) 0,
		(72) 0,
		(73) 0,
		(74) 0,
		(75) 0,
		(76) 0,
		(77) 0,
		(78) 0,
		(79) 0,
		(80) 0,
		(81) 0,
		(82) 0,
		(83) 0,
		(84) 0,
		(85) 0,
		(86) 0,
		(87) 0,
		(88) 0,
		(89) 0,
		(90) 0,
		(91) 0,
		(92) 0,
		(93) 0,
		(94) 0,
		(95) 0,
		(96) 0,
		(97) 0,
		(98) 0,
		(99) 0,
	],
}"
    );
}

#[test]
fn index_width_test_05() {
    let test_struct = IndexWidthTest {
        collection: vec![0; 101],
    };
    assert!(
        test_struct.obj2str(1, 2)
            == "IndexWidthTest {
	collection: [
		(  0) 0,
		(  1) 0,
		(  2) 0,
		(  3) 0,
		(  4) 0,
		(  5) 0,
		(  6) 0,
		(  7) 0,
		(  8) 0,
		(  9) 0,
		( 10) 0,
		( 11) 0,
		( 12) 0,
		( 13) 0,
		( 14) 0,
		( 15) 0,
		( 16) 0,
		( 17) 0,
		( 18) 0,
		( 19) 0,
		( 20) 0,
		( 21) 0,
		( 22) 0,
		( 23) 0,
		( 24) 0,
		( 25) 0,
		( 26) 0,
		( 27) 0,
		( 28) 0,
		( 29) 0,
		( 30) 0,
		( 31) 0,
		( 32) 0,
		( 33) 0,
		( 34) 0,
		( 35) 0,
		( 36) 0,
		( 37) 0,
		( 38) 0,
		( 39) 0,
		( 40) 0,
		( 41) 0,
		( 42) 0,
		( 43) 0,
		( 44) 0,
		( 45) 0,
		( 46) 0,
		( 47) 0,
		( 48) 0,
		( 49) 0,
		( 50) 0,
		( 51) 0,
		( 52) 0,
		( 53) 0,
		( 54) 0,
		( 55) 0,
		( 56) 0,
		( 57) 0,
		( 58) 0,
		( 59) 0,
		( 60) 0,
		( 61) 0,
		( 62) 0,
		( 63) 0,
		( 64) 0,
		( 65) 0,
		( 66) 0,
		( 67) 0,
		( 68) 0,
		( 69) 0,
		( 70) 0,
		( 71) 0,
		( 72) 0,
		( 73) 0,
		( 74) 0,
		( 75) 0,
		( 76) 0,
		( 77) 0,
		( 78) 0,
		( 79) 0,
		( 80) 0,
		( 81) 0,
		( 82) 0,
		( 83) 0,
		( 84) 0,
		( 85) 0,
		( 86) 0,
		( 87) 0,
		( 88) 0,
		( 89) 0,
		( 90) 0,
		( 91) 0,
		( 92) 0,
		( 93) 0,
		( 94) 0,
		( 95) 0,
		( 96) 0,
		( 97) 0,
		( 98) 0,
		( 99) 0,
		(100) 0,
	],
}"
    );
}
