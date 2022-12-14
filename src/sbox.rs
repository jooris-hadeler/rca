pub static SUBSTITUTION_BOX: [u8; 256] = [
    0x48, 0x40, 0x58, 0xB5, 0x08, 0x24, 0x29, 0x76, 0xF4, 0x72, 0x71, 0xDF, 0x91, 0x7E, 0x0D, 0x63,
    0x01, 0x4D, 0x5B, 0x11, 0xE8, 0x4E, 0x7D, 0xEA, 0x28, 0xE6, 0x64, 0x86, 0x88, 0x9C, 0xEB, 0x59,
    0xFD, 0x03, 0xE7, 0xA1, 0xC5, 0xA8, 0x32, 0x8B, 0x99, 0x5D, 0xAD, 0x9E, 0xF6, 0x14, 0x33, 0xB6,
    0xBF, 0xC3, 0xF1, 0xD3, 0xEE, 0xE5, 0xA6, 0x3E, 0x0F, 0x57, 0x1A, 0xA4, 0xB3, 0xCE, 0x51, 0xC2,
    0x1E, 0xE1, 0xD6, 0x8C, 0xDC, 0xF7, 0xE2, 0xFF, 0x19, 0x1D, 0xA7, 0x27, 0x8D, 0xBC, 0x79, 0x74,
    0xB8, 0xD0, 0xD4, 0x6C, 0x21, 0x53, 0xDB, 0x7A, 0x9D, 0x2F, 0x98, 0xDA, 0x89, 0xB4, 0x75, 0x4F,
    0x96, 0x2D, 0x0E, 0x52, 0xF0, 0x78, 0xA9, 0xA3, 0x26, 0xAE, 0x5E, 0x36, 0x9A, 0x4B, 0x68, 0xA5,
    0x60, 0x42, 0xE9, 0x73, 0x10, 0x25, 0xFB, 0x6A, 0xDD, 0x1B, 0xCD, 0xD5, 0x09, 0x82, 0xCF, 0x05,
    0xA2, 0xAC, 0xE3, 0xF8, 0x92, 0x0A, 0x93, 0xBB, 0x85, 0x04, 0x5F, 0xEC, 0x17, 0xC6, 0xF5, 0x8F,
    0x35, 0xAA, 0xCA, 0x94, 0xB0, 0x18, 0xD9, 0x67, 0x3B, 0x69, 0xB2, 0xC8, 0xC4, 0x3F, 0xEF, 0x45,
    0x16, 0x2B, 0x43, 0x2C, 0xDE, 0x4C, 0x07, 0xF2, 0x06, 0x20, 0xAB, 0xD7, 0x62, 0x34, 0x5A, 0x44,
    0x55, 0xB9, 0xD1, 0xCC, 0xC7, 0x7F, 0xAF, 0x54, 0x37, 0x9F, 0x1F, 0x0C, 0xC1, 0x80, 0x46, 0xBA,
    0xCB, 0xC9, 0x6F, 0x66, 0x97, 0x38, 0x31, 0x2E, 0x56, 0x39, 0x7C, 0x15, 0x3D, 0x3A, 0x83, 0x84,
    0x90, 0x70, 0x6B, 0xB7, 0x8E, 0xD2, 0x5C, 0xF3, 0xFA, 0x2A, 0xBD, 0x6E, 0x22, 0x12, 0x1C, 0x61,
    0x47, 0xA0, 0x49, 0x65, 0xBE, 0x95, 0x77, 0xFC, 0xB1, 0x3C, 0xFE, 0x6D, 0x41, 0x50, 0xF9, 0xD8,
    0x87, 0x0B, 0x30, 0x8A, 0xC0, 0x02, 0x4A, 0x23, 0xE0, 0xE4, 0x9B, 0x00, 0xED, 0x7B, 0x81, 0x13,
];

pub static INVERSE_SUBSTITUTION_BOX: [u8; 256] = [
    0xFB, 0x10, 0xF5, 0x21, 0x89, 0x7F, 0xA8, 0xA6, 0x04, 0x7C, 0x85, 0xF1, 0xBB, 0x0E, 0x62, 0x38,
    0x74, 0x13, 0xDD, 0xFF, 0x2D, 0xCB, 0xA0, 0x8C, 0x95, 0x48, 0x3A, 0x79, 0xDE, 0x49, 0x40, 0xBA,
    0xA9, 0x54, 0xDC, 0xF7, 0x05, 0x75, 0x68, 0x4B, 0x18, 0x06, 0xD9, 0xA1, 0xA3, 0x61, 0xC7, 0x59,
    0xF2, 0xC6, 0x26, 0x2E, 0xAD, 0x90, 0x6B, 0xB8, 0xC5, 0xC9, 0xCD, 0x98, 0xE9, 0xCC, 0x37, 0x9D,
    0x01, 0xEC, 0x71, 0xA2, 0xAF, 0x9F, 0xBE, 0xE0, 0x00, 0xE2, 0xF6, 0x6D, 0xA5, 0x11, 0x15, 0x5F,
    0xED, 0x3E, 0x63, 0x55, 0xB7, 0xB0, 0xC8, 0x39, 0x02, 0x1F, 0xAE, 0x12, 0xD6, 0x29, 0x6A, 0x8A,
    0x70, 0xDF, 0xAC, 0x0F, 0x1A, 0xE3, 0xC3, 0x97, 0x6E, 0x99, 0x77, 0xD2, 0x53, 0xEB, 0xDB, 0xC2,
    0xD1, 0x0A, 0x09, 0x73, 0x4F, 0x5E, 0x07, 0xE6, 0x65, 0x4E, 0x57, 0xFD, 0xCA, 0x16, 0x0D, 0xB5,
    0xBD, 0xFE, 0x7D, 0xCE, 0xCF, 0x88, 0x1B, 0xF0, 0x1C, 0x5C, 0xF3, 0x27, 0x43, 0x4C, 0xD4, 0x8F,
    0xD0, 0x0C, 0x84, 0x86, 0x93, 0xE5, 0x60, 0xC4, 0x5A, 0x28, 0x6C, 0xFA, 0x1D, 0x58, 0x2B, 0xB9,
    0xE1, 0x23, 0x80, 0x67, 0x3B, 0x6F, 0x36, 0x4A, 0x25, 0x66, 0x91, 0xAA, 0x81, 0x2A, 0x69, 0xB6,
    0x94, 0xE8, 0x9A, 0x3C, 0x5D, 0x03, 0x2F, 0xD3, 0x50, 0xB1, 0xBF, 0x87, 0x4D, 0xDA, 0xE4, 0x30,
    0xF4, 0xBC, 0x3F, 0x31, 0x9C, 0x24, 0x8D, 0xB4, 0x9B, 0xC1, 0x92, 0xC0, 0xB3, 0x7A, 0x3D, 0x7E,
    0x51, 0xB2, 0xD5, 0x33, 0x52, 0x7B, 0x42, 0xAB, 0xEF, 0x96, 0x5B, 0x56, 0x44, 0x78, 0xA4, 0x0B,
    0xF8, 0x41, 0x46, 0x82, 0xF9, 0x35, 0x19, 0x22, 0x14, 0x72, 0x17, 0x1E, 0x8B, 0xFC, 0x34, 0x9E,
    0x64, 0x32, 0xA7, 0xD7, 0x08, 0x8E, 0x2C, 0x45, 0x83, 0xEE, 0xD8, 0x76, 0xE7, 0x20, 0xEA, 0x47,
];
