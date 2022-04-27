extern crate core;


use ironjvm_cfparser::ClassFileParser;

#[test]
fn hello_world() {
    let bytes =
        std::fs::read("../test_classes/com/github/htgazurex1212/ironjvm/tests/HelloWorld.class").unwrap();

    let mut parser = ClassFileParser::new(bytes.as_slice());
    let result = parser.parse();
    if let Err(error) = result {
        panic!("failed to parse classfile: {error:?}");
    }

    let classfile = result.unwrap();
    let expect = expect_test::expect![["
ClassFile {
    magic: 3405691582,
    minor_version: 0,
    major_version: 62,
    constant_pool_count: 29,
    constant_pool: [
        CpInfo {
            tag: 10,
            info: ConstantMethodRef {
                class_index: 2,
                name_and_type_index: 3,
            },
        },
        CpInfo {
            tag: 7,
            info: ConstantClass {
                name_index: 4,
            },
        },
        CpInfo {
            tag: 12,
            info: ConstantNameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 16,
                bytes: [
                    106,
                    97,
                    118,
                    97,
                    47,
                    108,
                    97,
                    110,
                    103,
                    47,
                    79,
                    98,
                    106,
                    101,
                    99,
                    116,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 6,
                bytes: [
                    60,
                    105,
                    110,
                    105,
                    116,
                    62,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 3,
                bytes: [
                    40,
                    41,
                    86,
                ],
            },
        },
        CpInfo {
            tag: 9,
            info: ConstantFieldRef {
                class_index: 8,
                name_and_type_index: 9,
            },
        },
        CpInfo {
            tag: 7,
            info: ConstantClass {
                name_index: 10,
            },
        },
        CpInfo {
            tag: 12,
            info: ConstantNameAndType {
                name_index: 11,
                descriptor_index: 12,
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 16,
                bytes: [
                    106,
                    97,
                    118,
                    97,
                    47,
                    108,
                    97,
                    110,
                    103,
                    47,
                    83,
                    121,
                    115,
                    116,
                    101,
                    109,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 3,
                bytes: [
                    111,
                    117,
                    116,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 21,
                bytes: [
                    76,
                    106,
                    97,
                    118,
                    97,
                    47,
                    105,
                    111,
                    47,
                    80,
                    114,
                    105,
                    110,
                    116,
                    83,
                    116,
                    114,
                    101,
                    97,
                    109,
                    59,
                ],
            },
        },
        CpInfo {
            tag: 8,
            info: ConstantString {
                string_index: 14,
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 24,
                bytes: [
                    72,
                    101,
                    108,
                    108,
                    111,
                    44,
                    32,
                    119,
                    111,
                    114,
                    108,
                    100,
                    32,
                    105,
                    110,
                    32,
                    73,
                    114,
                    111,
                    110,
                    74,
                    86,
                    77,
                    33,
                ],
            },
        },
        CpInfo {
            tag: 10,
            info: ConstantMethodRef {
                class_index: 16,
                name_and_type_index: 17,
            },
        },
        CpInfo {
            tag: 7,
            info: ConstantClass {
                name_index: 18,
            },
        },
        CpInfo {
            tag: 12,
            info: ConstantNameAndType {
                name_index: 19,
                descriptor_index: 20,
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 19,
                bytes: [
                    106,
                    97,
                    118,
                    97,
                    47,
                    105,
                    111,
                    47,
                    80,
                    114,
                    105,
                    110,
                    116,
                    83,
                    116,
                    114,
                    101,
                    97,
                    109,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 7,
                bytes: [
                    112,
                    114,
                    105,
                    110,
                    116,
                    108,
                    110,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 21,
                bytes: [
                    40,
                    76,
                    106,
                    97,
                    118,
                    97,
                    47,
                    108,
                    97,
                    110,
                    103,
                    47,
                    83,
                    116,
                    114,
                    105,
                    110,
                    103,
                    59,
                    41,
                    86,
                ],
            },
        },
        CpInfo {
            tag: 7,
            info: ConstantClass {
                name_index: 22,
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 49,
                bytes: [
                    99,
                    111,
                    109,
                    47,
                    103,
                    105,
                    116,
                    104,
                    117,
                    98,
                    47,
                    104,
                    116,
                    103,
                    97,
                    122,
                    117,
                    114,
                    101,
                    120,
                    49,
                    50,
                    49,
                    50,
                    47,
                    105,
                    114,
                    111,
                    110,
                    106,
                    118,
                    109,
                    47,
                    116,
                    101,
                    115,
                    116,
                    115,
                    47,
                    72,
                    101,
                    108,
                    108,
                    111,
                    87,
                    111,
                    114,
                    108,
                    100,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 4,
                bytes: [
                    67,
                    111,
                    100,
                    101,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 15,
                bytes: [
                    76,
                    105,
                    110,
                    101,
                    78,
                    117,
                    109,
                    98,
                    101,
                    114,
                    84,
                    97,
                    98,
                    108,
                    101,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 4,
                bytes: [
                    109,
                    97,
                    105,
                    110,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 22,
                bytes: [
                    40,
                    91,
                    76,
                    106,
                    97,
                    118,
                    97,
                    47,
                    108,
                    97,
                    110,
                    103,
                    47,
                    83,
                    116,
                    114,
                    105,
                    110,
                    103,
                    59,
                    41,
                    86,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 10,
                bytes: [
                    83,
                    111,
                    117,
                    114,
                    99,
                    101,
                    70,
                    105,
                    108,
                    101,
                ],
            },
        },
        CpInfo {
            tag: 1,
            info: ConstantUtf8 {
                length: 15,
                bytes: [
                    72,
                    101,
                    108,
                    108,
                    111,
                    87,
                    111,
                    114,
                    108,
                    100,
                    46,
                    106,
                    97,
                    118,
                    97,
                ],
            },
        },
    ],
    access_flags: 33,
    this_class: 21,
    super_class: 2,
    interfaces_count: 0,
    interfaces: [],
    fields_count: 0,
    fields: [],
    methods_count: 2,
    methods: [
        MethodInfo {
            access_flags: 1,
            name_index: 5,
            descriptor_index: 6,
            attributes_count: 1,
            attributes: [
                AttributeInfo {
                    attribute_name_index: 23,
                    attribute_length: 29,
                    info: CodeAttribute {
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 5,
                        code: [
                            42,
                            183,
                            0,
                            1,
                            177,
                        ],
                        exception_table_length: 0,
                        exception_table: [],
                        attributes_count: 1,
                        attributes: [
                            AttributeInfo {
                                attribute_name_index: 24,
                                attribute_length: 6,
                                info: LineNumberTableAttribute {
                                    line_number_table_length: 1,
                                    line_number_table: [
                                        LineNumber {
                                            start_pc: 0,
                                            line_number: 3,
                                        },
                                    ],
                                },
                            },
                        ],
                    },
                },
            ],
        },
        MethodInfo {
            access_flags: 9,
            name_index: 25,
            descriptor_index: 26,
            attributes_count: 1,
            attributes: [
                AttributeInfo {
                    attribute_name_index: 23,
                    attribute_length: 37,
                    info: CodeAttribute {
                        max_stack: 2,
                        max_locals: 1,
                        code_length: 9,
                        code: [
                            178,
                            0,
                            7,
                            18,
                            13,
                            182,
                            0,
                            15,
                            177,
                        ],
                        exception_table_length: 0,
                        exception_table: [],
                        attributes_count: 1,
                        attributes: [
                            AttributeInfo {
                                attribute_name_index: 24,
                                attribute_length: 10,
                                info: LineNumberTableAttribute {
                                    line_number_table_length: 2,
                                    line_number_table: [
                                        LineNumber {
                                            start_pc: 0,
                                            line_number: 5,
                                        },
                                        LineNumber {
                                            start_pc: 8,
                                            line_number: 6,
                                        },
                                    ],
                                },
                            },
                        ],
                    },
                },
            ],
        },
    ],
    attributes_count: 1,
    attributes: [
        AttributeInfo {
            attribute_name_index: 27,
            attribute_length: 2,
            info: SourceFileAttribute {
                sourcefile_index: 28,
            },
        },
    ],
}
"]];
    expect.assert_debug_eq(&classfile);
}
