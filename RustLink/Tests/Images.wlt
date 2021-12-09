Needs["MUnit`"]

Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_image_arg",
        {LibraryDataType[Image, "Bit"]},
        NumericArray
    ][
        Image[{{0, 1}, {1, 0}}, "Bit"]
    ]
    ,
    NumericArray[{0, 1, 1, 0}, "Integer8"]
]

Test[
    LibraryFunctionLoad["liblibrary_tests", "test_create_bitmap_image", {}, Image][]
    ,
    Image[{{0, 1}, {1, 0}}, "Bit"]
]

Test[
    LibraryFunctionLoad["liblibrary_tests", "test_create_color_rgb_u8_image", {}, Image][]
    ,
    Image[
        NumericArray[
            {
                {{255, 0  }, {  0, 200}}, (* Red channel*)
                {{0,   255}, {  0, 200}}, (* Green channel *)
                {{0,     0}, {255, 200}}  (* Blue channel *)
            },
            "UnsignedInteger8"
        ],
        "Byte",
        ColorSpace -> "RGB",
        Interleaving -> False
    ]
]

Test[
    LibraryFunctionLoad["liblibrary_tests", "test_create_color_rgb_f32_image", {}, Image][]
    ,
    Image[
        NumericArray[
            {
                {{1.0, 0.0}, {0.0, 0.8}}, (* Red channel*)
                {{0.0, 1.0}, {0.0, 0.8}}, (* Green channel *)
                {{0.0, 0.0}, {1.0, 0.8}}  (* Blue channel *)
            },
            "Real32"
        ],
        "Real32",
        ColorSpace -> "RGB",
        Interleaving -> False
    ]
]