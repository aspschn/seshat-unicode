pub(crate) fn wspace(cp: u32) -> bool {
    if (0x0009..0x000D + 1).contains(&cp) {
        return true;
    }
    if (0x0020..0x0020 + 1).contains(&cp) {
        return true;
    }
    if (0x0085..0x0085 + 1).contains(&cp) {
        return true;
    }
    if (0x00A0..0x00A0 + 1).contains(&cp) {
        return true;
    }
    if (0x1680..0x1680 + 1).contains(&cp) {
        return true;
    }
    if (0x2000..0x200A + 1).contains(&cp) {
        return true;
    }
    if (0x2028..0x2028 + 1).contains(&cp) {
        return true;
    }
    if (0x2029..0x2029 + 1).contains(&cp) {
        return true;
    }
    if (0x202F..0x202F + 1).contains(&cp) {
        return true;
    }
    if (0x205F..0x205F + 1).contains(&cp) {
        return true;
    }
    if (0x3000..0x3000 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn bidi_c(cp: u32) -> bool {
    if (0x061C..0x061C + 1).contains(&cp) {
        return true;
    }
    if (0x200E..0x200F + 1).contains(&cp) {
        return true;
    }
    if (0x202A..0x202E + 1).contains(&cp) {
        return true;
    }
    if (0x2066..0x2069 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn join_c(cp: u32) -> bool {
    if (0x200C..0x200D + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn dash(cp: u32) -> bool {
    if (0x002D..0x002D + 1).contains(&cp) {
        return true;
    }
    if (0x058A..0x058A + 1).contains(&cp) {
        return true;
    }
    if (0x05BE..0x05BE + 1).contains(&cp) {
        return true;
    }
    if (0x1400..0x1400 + 1).contains(&cp) {
        return true;
    }
    if (0x1806..0x1806 + 1).contains(&cp) {
        return true;
    }
    if (0x2010..0x2015 + 1).contains(&cp) {
        return true;
    }
    if (0x2053..0x2053 + 1).contains(&cp) {
        return true;
    }
    if (0x207B..0x207B + 1).contains(&cp) {
        return true;
    }
    if (0x208B..0x208B + 1).contains(&cp) {
        return true;
    }
    if (0x2212..0x2212 + 1).contains(&cp) {
        return true;
    }
    if (0x2E17..0x2E17 + 1).contains(&cp) {
        return true;
    }
    if (0x2E1A..0x2E1A + 1).contains(&cp) {
        return true;
    }
    if (0x2E3A..0x2E3B + 1).contains(&cp) {
        return true;
    }
    if (0x2E40..0x2E40 + 1).contains(&cp) {
        return true;
    }
    if (0x2E5D..0x2E5D + 1).contains(&cp) {
        return true;
    }
    if (0x301C..0x301C + 1).contains(&cp) {
        return true;
    }
    if (0x3030..0x3030 + 1).contains(&cp) {
        return true;
    }
    if (0x30A0..0x30A0 + 1).contains(&cp) {
        return true;
    }
    if (0xFE31..0xFE32 + 1).contains(&cp) {
        return true;
    }
    if (0xFE58..0xFE58 + 1).contains(&cp) {
        return true;
    }
    if (0xFE63..0xFE63 + 1).contains(&cp) {
        return true;
    }
    if (0xFF0D..0xFF0D + 1).contains(&cp) {
        return true;
    }
    if (0x10EAD..0x10EAD + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn hyphen(cp: u32) -> bool {
    if (0x002D..0x002D + 1).contains(&cp) {
        return true;
    }
    if (0x00AD..0x00AD + 1).contains(&cp) {
        return true;
    }
    if (0x058A..0x058A + 1).contains(&cp) {
        return true;
    }
    if (0x1806..0x1806 + 1).contains(&cp) {
        return true;
    }
    if (0x2010..0x2011 + 1).contains(&cp) {
        return true;
    }
    if (0x2E17..0x2E17 + 1).contains(&cp) {
        return true;
    }
    if (0x30FB..0x30FB + 1).contains(&cp) {
        return true;
    }
    if (0xFE63..0xFE63 + 1).contains(&cp) {
        return true;
    }
    if (0xFF0D..0xFF0D + 1).contains(&cp) {
        return true;
    }
    if (0xFF65..0xFF65 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn qmark(cp: u32) -> bool {
    if (0x0022..0x0022 + 1).contains(&cp) {
        return true;
    }
    if (0x0027..0x0027 + 1).contains(&cp) {
        return true;
    }
    if (0x00AB..0x00AB + 1).contains(&cp) {
        return true;
    }
    if (0x00BB..0x00BB + 1).contains(&cp) {
        return true;
    }
    if (0x2018..0x2018 + 1).contains(&cp) {
        return true;
    }
    if (0x2019..0x2019 + 1).contains(&cp) {
        return true;
    }
    if (0x201A..0x201A + 1).contains(&cp) {
        return true;
    }
    if (0x201B..0x201C + 1).contains(&cp) {
        return true;
    }
    if (0x201D..0x201D + 1).contains(&cp) {
        return true;
    }
    if (0x201E..0x201E + 1).contains(&cp) {
        return true;
    }
    if (0x201F..0x201F + 1).contains(&cp) {
        return true;
    }
    if (0x2039..0x2039 + 1).contains(&cp) {
        return true;
    }
    if (0x203A..0x203A + 1).contains(&cp) {
        return true;
    }
    if (0x2E42..0x2E42 + 1).contains(&cp) {
        return true;
    }
    if (0x300C..0x300C + 1).contains(&cp) {
        return true;
    }
    if (0x300D..0x300D + 1).contains(&cp) {
        return true;
    }
    if (0x300E..0x300E + 1).contains(&cp) {
        return true;
    }
    if (0x300F..0x300F + 1).contains(&cp) {
        return true;
    }
    if (0x301D..0x301D + 1).contains(&cp) {
        return true;
    }
    if (0x301E..0x301F + 1).contains(&cp) {
        return true;
    }
    if (0xFE41..0xFE41 + 1).contains(&cp) {
        return true;
    }
    if (0xFE42..0xFE42 + 1).contains(&cp) {
        return true;
    }
    if (0xFE43..0xFE43 + 1).contains(&cp) {
        return true;
    }
    if (0xFE44..0xFE44 + 1).contains(&cp) {
        return true;
    }
    if (0xFF02..0xFF02 + 1).contains(&cp) {
        return true;
    }
    if (0xFF07..0xFF07 + 1).contains(&cp) {
        return true;
    }
    if (0xFF62..0xFF62 + 1).contains(&cp) {
        return true;
    }
    if (0xFF63..0xFF63 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn term(cp: u32) -> bool {
    if (0x0021..0x0021 + 1).contains(&cp) {
        return true;
    }
    if (0x002C..0x002C + 1).contains(&cp) {
        return true;
    }
    if (0x002E..0x002E + 1).contains(&cp) {
        return true;
    }
    if (0x003A..0x003B + 1).contains(&cp) {
        return true;
    }
    if (0x003F..0x003F + 1).contains(&cp) {
        return true;
    }
    if (0x037E..0x037E + 1).contains(&cp) {
        return true;
    }
    if (0x0387..0x0387 + 1).contains(&cp) {
        return true;
    }
    if (0x0589..0x0589 + 1).contains(&cp) {
        return true;
    }
    if (0x05C3..0x05C3 + 1).contains(&cp) {
        return true;
    }
    if (0x060C..0x060C + 1).contains(&cp) {
        return true;
    }
    if (0x061B..0x061B + 1).contains(&cp) {
        return true;
    }
    if (0x061D..0x061F + 1).contains(&cp) {
        return true;
    }
    if (0x06D4..0x06D4 + 1).contains(&cp) {
        return true;
    }
    if (0x0700..0x070A + 1).contains(&cp) {
        return true;
    }
    if (0x070C..0x070C + 1).contains(&cp) {
        return true;
    }
    if (0x07F8..0x07F9 + 1).contains(&cp) {
        return true;
    }
    if (0x0830..0x083E + 1).contains(&cp) {
        return true;
    }
    if (0x085E..0x085E + 1).contains(&cp) {
        return true;
    }
    if (0x0964..0x0965 + 1).contains(&cp) {
        return true;
    }
    if (0x0E5A..0x0E5B + 1).contains(&cp) {
        return true;
    }
    if (0x0F08..0x0F08 + 1).contains(&cp) {
        return true;
    }
    if (0x0F0D..0x0F12 + 1).contains(&cp) {
        return true;
    }
    if (0x104A..0x104B + 1).contains(&cp) {
        return true;
    }
    if (0x1361..0x1368 + 1).contains(&cp) {
        return true;
    }
    if (0x166E..0x166E + 1).contains(&cp) {
        return true;
    }
    if (0x16EB..0x16ED + 1).contains(&cp) {
        return true;
    }
    if (0x1735..0x1736 + 1).contains(&cp) {
        return true;
    }
    if (0x17D4..0x17D6 + 1).contains(&cp) {
        return true;
    }
    if (0x17DA..0x17DA + 1).contains(&cp) {
        return true;
    }
    if (0x1802..0x1805 + 1).contains(&cp) {
        return true;
    }
    if (0x1808..0x1809 + 1).contains(&cp) {
        return true;
    }
    if (0x1944..0x1945 + 1).contains(&cp) {
        return true;
    }
    if (0x1AA8..0x1AAB + 1).contains(&cp) {
        return true;
    }
    if (0x1B5A..0x1B5B + 1).contains(&cp) {
        return true;
    }
    if (0x1B5D..0x1B5F + 1).contains(&cp) {
        return true;
    }
    if (0x1B7D..0x1B7E + 1).contains(&cp) {
        return true;
    }
    if (0x1C3B..0x1C3F + 1).contains(&cp) {
        return true;
    }
    if (0x1C7E..0x1C7F + 1).contains(&cp) {
        return true;
    }
    if (0x203C..0x203D + 1).contains(&cp) {
        return true;
    }
    if (0x2047..0x2049 + 1).contains(&cp) {
        return true;
    }
    if (0x2E2E..0x2E2E + 1).contains(&cp) {
        return true;
    }
    if (0x2E3C..0x2E3C + 1).contains(&cp) {
        return true;
    }
    if (0x2E41..0x2E41 + 1).contains(&cp) {
        return true;
    }
    if (0x2E4C..0x2E4C + 1).contains(&cp) {
        return true;
    }
    if (0x2E4E..0x2E4F + 1).contains(&cp) {
        return true;
    }
    if (0x2E53..0x2E54 + 1).contains(&cp) {
        return true;
    }
    if (0x3001..0x3002 + 1).contains(&cp) {
        return true;
    }
    if (0xA4FE..0xA4FF + 1).contains(&cp) {
        return true;
    }
    if (0xA60D..0xA60F + 1).contains(&cp) {
        return true;
    }
    if (0xA6F3..0xA6F7 + 1).contains(&cp) {
        return true;
    }
    if (0xA876..0xA877 + 1).contains(&cp) {
        return true;
    }
    if (0xA8CE..0xA8CF + 1).contains(&cp) {
        return true;
    }
    if (0xA92F..0xA92F + 1).contains(&cp) {
        return true;
    }
    if (0xA9C7..0xA9C9 + 1).contains(&cp) {
        return true;
    }
    if (0xAA5D..0xAA5F + 1).contains(&cp) {
        return true;
    }
    if (0xAADF..0xAADF + 1).contains(&cp) {
        return true;
    }
    if (0xAAF0..0xAAF1 + 1).contains(&cp) {
        return true;
    }
    if (0xABEB..0xABEB + 1).contains(&cp) {
        return true;
    }
    if (0xFE50..0xFE52 + 1).contains(&cp) {
        return true;
    }
    if (0xFE54..0xFE57 + 1).contains(&cp) {
        return true;
    }
    if (0xFF01..0xFF01 + 1).contains(&cp) {
        return true;
    }
    if (0xFF0C..0xFF0C + 1).contains(&cp) {
        return true;
    }
    if (0xFF0E..0xFF0E + 1).contains(&cp) {
        return true;
    }
    if (0xFF1A..0xFF1B + 1).contains(&cp) {
        return true;
    }
    if (0xFF1F..0xFF1F + 1).contains(&cp) {
        return true;
    }
    if (0xFF61..0xFF61 + 1).contains(&cp) {
        return true;
    }
    if (0xFF64..0xFF64 + 1).contains(&cp) {
        return true;
    }
    if (0x1039F..0x1039F + 1).contains(&cp) {
        return true;
    }
    if (0x103D0..0x103D0 + 1).contains(&cp) {
        return true;
    }
    if (0x10857..0x10857 + 1).contains(&cp) {
        return true;
    }
    if (0x1091F..0x1091F + 1).contains(&cp) {
        return true;
    }
    if (0x10A56..0x10A57 + 1).contains(&cp) {
        return true;
    }
    if (0x10AF0..0x10AF5 + 1).contains(&cp) {
        return true;
    }
    if (0x10B3A..0x10B3F + 1).contains(&cp) {
        return true;
    }
    if (0x10B99..0x10B9C + 1).contains(&cp) {
        return true;
    }
    if (0x10F55..0x10F59 + 1).contains(&cp) {
        return true;
    }
    if (0x10F86..0x10F89 + 1).contains(&cp) {
        return true;
    }
    if (0x11047..0x1104D + 1).contains(&cp) {
        return true;
    }
    if (0x110BE..0x110C1 + 1).contains(&cp) {
        return true;
    }
    if (0x11141..0x11143 + 1).contains(&cp) {
        return true;
    }
    if (0x111C5..0x111C6 + 1).contains(&cp) {
        return true;
    }
    if (0x111CD..0x111CD + 1).contains(&cp) {
        return true;
    }
    if (0x111DE..0x111DF + 1).contains(&cp) {
        return true;
    }
    if (0x11238..0x1123C + 1).contains(&cp) {
        return true;
    }
    if (0x112A9..0x112A9 + 1).contains(&cp) {
        return true;
    }
    if (0x1144B..0x1144D + 1).contains(&cp) {
        return true;
    }
    if (0x1145A..0x1145B + 1).contains(&cp) {
        return true;
    }
    if (0x115C2..0x115C5 + 1).contains(&cp) {
        return true;
    }
    if (0x115C9..0x115D7 + 1).contains(&cp) {
        return true;
    }
    if (0x11641..0x11642 + 1).contains(&cp) {
        return true;
    }
    if (0x1173C..0x1173E + 1).contains(&cp) {
        return true;
    }
    if (0x11944..0x11944 + 1).contains(&cp) {
        return true;
    }
    if (0x11946..0x11946 + 1).contains(&cp) {
        return true;
    }
    if (0x11A42..0x11A43 + 1).contains(&cp) {
        return true;
    }
    if (0x11A9B..0x11A9C + 1).contains(&cp) {
        return true;
    }
    if (0x11AA1..0x11AA2 + 1).contains(&cp) {
        return true;
    }
    if (0x11C41..0x11C43 + 1).contains(&cp) {
        return true;
    }
    if (0x11C71..0x11C71 + 1).contains(&cp) {
        return true;
    }
    if (0x11EF7..0x11EF8 + 1).contains(&cp) {
        return true;
    }
    if (0x11F43..0x11F44 + 1).contains(&cp) {
        return true;
    }
    if (0x12470..0x12474 + 1).contains(&cp) {
        return true;
    }
    if (0x16A6E..0x16A6F + 1).contains(&cp) {
        return true;
    }
    if (0x16AF5..0x16AF5 + 1).contains(&cp) {
        return true;
    }
    if (0x16B37..0x16B39 + 1).contains(&cp) {
        return true;
    }
    if (0x16B44..0x16B44 + 1).contains(&cp) {
        return true;
    }
    if (0x16E97..0x16E98 + 1).contains(&cp) {
        return true;
    }
    if (0x1BC9F..0x1BC9F + 1).contains(&cp) {
        return true;
    }
    if (0x1DA87..0x1DA8A + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn omath(cp: u32) -> bool {
    if (0x005E..0x005E + 1).contains(&cp) {
        return true;
    }
    if (0x03D0..0x03D2 + 1).contains(&cp) {
        return true;
    }
    if (0x03D5..0x03D5 + 1).contains(&cp) {
        return true;
    }
    if (0x03F0..0x03F1 + 1).contains(&cp) {
        return true;
    }
    if (0x03F4..0x03F5 + 1).contains(&cp) {
        return true;
    }
    if (0x2016..0x2016 + 1).contains(&cp) {
        return true;
    }
    if (0x2032..0x2034 + 1).contains(&cp) {
        return true;
    }
    if (0x2040..0x2040 + 1).contains(&cp) {
        return true;
    }
    if (0x2061..0x2064 + 1).contains(&cp) {
        return true;
    }
    if (0x207D..0x207D + 1).contains(&cp) {
        return true;
    }
    if (0x207E..0x207E + 1).contains(&cp) {
        return true;
    }
    if (0x208D..0x208D + 1).contains(&cp) {
        return true;
    }
    if (0x208E..0x208E + 1).contains(&cp) {
        return true;
    }
    if (0x20D0..0x20DC + 1).contains(&cp) {
        return true;
    }
    if (0x20E1..0x20E1 + 1).contains(&cp) {
        return true;
    }
    if (0x20E5..0x20E6 + 1).contains(&cp) {
        return true;
    }
    if (0x20EB..0x20EF + 1).contains(&cp) {
        return true;
    }
    if (0x2102..0x2102 + 1).contains(&cp) {
        return true;
    }
    if (0x2107..0x2107 + 1).contains(&cp) {
        return true;
    }
    if (0x210A..0x2113 + 1).contains(&cp) {
        return true;
    }
    if (0x2115..0x2115 + 1).contains(&cp) {
        return true;
    }
    if (0x2119..0x211D + 1).contains(&cp) {
        return true;
    }
    if (0x2124..0x2124 + 1).contains(&cp) {
        return true;
    }
    if (0x2128..0x2128 + 1).contains(&cp) {
        return true;
    }
    if (0x2129..0x2129 + 1).contains(&cp) {
        return true;
    }
    if (0x212C..0x212D + 1).contains(&cp) {
        return true;
    }
    if (0x212F..0x2131 + 1).contains(&cp) {
        return true;
    }
    if (0x2133..0x2134 + 1).contains(&cp) {
        return true;
    }
    if (0x2135..0x2138 + 1).contains(&cp) {
        return true;
    }
    if (0x213C..0x213F + 1).contains(&cp) {
        return true;
    }
    if (0x2145..0x2149 + 1).contains(&cp) {
        return true;
    }
    if (0x2195..0x2199 + 1).contains(&cp) {
        return true;
    }
    if (0x219C..0x219F + 1).contains(&cp) {
        return true;
    }
    if (0x21A1..0x21A2 + 1).contains(&cp) {
        return true;
    }
    if (0x21A4..0x21A5 + 1).contains(&cp) {
        return true;
    }
    if (0x21A7..0x21A7 + 1).contains(&cp) {
        return true;
    }
    if (0x21A9..0x21AD + 1).contains(&cp) {
        return true;
    }
    if (0x21B0..0x21B1 + 1).contains(&cp) {
        return true;
    }
    if (0x21B6..0x21B7 + 1).contains(&cp) {
        return true;
    }
    if (0x21BC..0x21CD + 1).contains(&cp) {
        return true;
    }
    if (0x21D0..0x21D1 + 1).contains(&cp) {
        return true;
    }
    if (0x21D3..0x21D3 + 1).contains(&cp) {
        return true;
    }
    if (0x21D5..0x21DB + 1).contains(&cp) {
        return true;
    }
    if (0x21DD..0x21DD + 1).contains(&cp) {
        return true;
    }
    if (0x21E4..0x21E5 + 1).contains(&cp) {
        return true;
    }
    if (0x2308..0x2308 + 1).contains(&cp) {
        return true;
    }
    if (0x2309..0x2309 + 1).contains(&cp) {
        return true;
    }
    if (0x230A..0x230A + 1).contains(&cp) {
        return true;
    }
    if (0x230B..0x230B + 1).contains(&cp) {
        return true;
    }
    if (0x23B4..0x23B5 + 1).contains(&cp) {
        return true;
    }
    if (0x23B7..0x23B7 + 1).contains(&cp) {
        return true;
    }
    if (0x23D0..0x23D0 + 1).contains(&cp) {
        return true;
    }
    if (0x23E2..0x23E2 + 1).contains(&cp) {
        return true;
    }
    if (0x25A0..0x25A1 + 1).contains(&cp) {
        return true;
    }
    if (0x25AE..0x25B6 + 1).contains(&cp) {
        return true;
    }
    if (0x25BC..0x25C0 + 1).contains(&cp) {
        return true;
    }
    if (0x25C6..0x25C7 + 1).contains(&cp) {
        return true;
    }
    if (0x25CA..0x25CB + 1).contains(&cp) {
        return true;
    }
    if (0x25CF..0x25D3 + 1).contains(&cp) {
        return true;
    }
    if (0x25E2..0x25E2 + 1).contains(&cp) {
        return true;
    }
    if (0x25E4..0x25E4 + 1).contains(&cp) {
        return true;
    }
    if (0x25E7..0x25EC + 1).contains(&cp) {
        return true;
    }
    if (0x2605..0x2606 + 1).contains(&cp) {
        return true;
    }
    if (0x2640..0x2640 + 1).contains(&cp) {
        return true;
    }
    if (0x2642..0x2642 + 1).contains(&cp) {
        return true;
    }
    if (0x2660..0x2663 + 1).contains(&cp) {
        return true;
    }
    if (0x266D..0x266E + 1).contains(&cp) {
        return true;
    }
    if (0x27C5..0x27C5 + 1).contains(&cp) {
        return true;
    }
    if (0x27C6..0x27C6 + 1).contains(&cp) {
        return true;
    }
    if (0x27E6..0x27E6 + 1).contains(&cp) {
        return true;
    }
    if (0x27E7..0x27E7 + 1).contains(&cp) {
        return true;
    }
    if (0x27E8..0x27E8 + 1).contains(&cp) {
        return true;
    }
    if (0x27E9..0x27E9 + 1).contains(&cp) {
        return true;
    }
    if (0x27EA..0x27EA + 1).contains(&cp) {
        return true;
    }
    if (0x27EB..0x27EB + 1).contains(&cp) {
        return true;
    }
    if (0x27EC..0x27EC + 1).contains(&cp) {
        return true;
    }
    if (0x27ED..0x27ED + 1).contains(&cp) {
        return true;
    }
    if (0x27EE..0x27EE + 1).contains(&cp) {
        return true;
    }
    if (0x27EF..0x27EF + 1).contains(&cp) {
        return true;
    }
    if (0x2983..0x2983 + 1).contains(&cp) {
        return true;
    }
    if (0x2984..0x2984 + 1).contains(&cp) {
        return true;
    }
    if (0x2985..0x2985 + 1).contains(&cp) {
        return true;
    }
    if (0x2986..0x2986 + 1).contains(&cp) {
        return true;
    }
    if (0x2987..0x2987 + 1).contains(&cp) {
        return true;
    }
    if (0x2988..0x2988 + 1).contains(&cp) {
        return true;
    }
    if (0x2989..0x2989 + 1).contains(&cp) {
        return true;
    }
    if (0x298A..0x298A + 1).contains(&cp) {
        return true;
    }
    if (0x298B..0x298B + 1).contains(&cp) {
        return true;
    }
    if (0x298C..0x298C + 1).contains(&cp) {
        return true;
    }
    if (0x298D..0x298D + 1).contains(&cp) {
        return true;
    }
    if (0x298E..0x298E + 1).contains(&cp) {
        return true;
    }
    if (0x298F..0x298F + 1).contains(&cp) {
        return true;
    }
    if (0x2990..0x2990 + 1).contains(&cp) {
        return true;
    }
    if (0x2991..0x2991 + 1).contains(&cp) {
        return true;
    }
    if (0x2992..0x2992 + 1).contains(&cp) {
        return true;
    }
    if (0x2993..0x2993 + 1).contains(&cp) {
        return true;
    }
    if (0x2994..0x2994 + 1).contains(&cp) {
        return true;
    }
    if (0x2995..0x2995 + 1).contains(&cp) {
        return true;
    }
    if (0x2996..0x2996 + 1).contains(&cp) {
        return true;
    }
    if (0x2997..0x2997 + 1).contains(&cp) {
        return true;
    }
    if (0x2998..0x2998 + 1).contains(&cp) {
        return true;
    }
    if (0x29D8..0x29D8 + 1).contains(&cp) {
        return true;
    }
    if (0x29D9..0x29D9 + 1).contains(&cp) {
        return true;
    }
    if (0x29DA..0x29DA + 1).contains(&cp) {
        return true;
    }
    if (0x29DB..0x29DB + 1).contains(&cp) {
        return true;
    }
    if (0x29FC..0x29FC + 1).contains(&cp) {
        return true;
    }
    if (0x29FD..0x29FD + 1).contains(&cp) {
        return true;
    }
    if (0xFE61..0xFE61 + 1).contains(&cp) {
        return true;
    }
    if (0xFE63..0xFE63 + 1).contains(&cp) {
        return true;
    }
    if (0xFE68..0xFE68 + 1).contains(&cp) {
        return true;
    }
    if (0xFF3C..0xFF3C + 1).contains(&cp) {
        return true;
    }
    if (0xFF3E..0xFF3E + 1).contains(&cp) {
        return true;
    }
    if (0x1D400..0x1D454 + 1).contains(&cp) {
        return true;
    }
    if (0x1D456..0x1D49C + 1).contains(&cp) {
        return true;
    }
    if (0x1D49E..0x1D49F + 1).contains(&cp) {
        return true;
    }
    if (0x1D4A2..0x1D4A2 + 1).contains(&cp) {
        return true;
    }
    if (0x1D4A5..0x1D4A6 + 1).contains(&cp) {
        return true;
    }
    if (0x1D4A9..0x1D4AC + 1).contains(&cp) {
        return true;
    }
    if (0x1D4AE..0x1D4B9 + 1).contains(&cp) {
        return true;
    }
    if (0x1D4BB..0x1D4BB + 1).contains(&cp) {
        return true;
    }
    if (0x1D4BD..0x1D4C3 + 1).contains(&cp) {
        return true;
    }
    if (0x1D4C5..0x1D505 + 1).contains(&cp) {
        return true;
    }
    if (0x1D507..0x1D50A + 1).contains(&cp) {
        return true;
    }
    if (0x1D50D..0x1D514 + 1).contains(&cp) {
        return true;
    }
    if (0x1D516..0x1D51C + 1).contains(&cp) {
        return true;
    }
    if (0x1D51E..0x1D539 + 1).contains(&cp) {
        return true;
    }
    if (0x1D53B..0x1D53E + 1).contains(&cp) {
        return true;
    }
    if (0x1D540..0x1D544 + 1).contains(&cp) {
        return true;
    }
    if (0x1D546..0x1D546 + 1).contains(&cp) {
        return true;
    }
    if (0x1D54A..0x1D550 + 1).contains(&cp) {
        return true;
    }
    if (0x1D552..0x1D6A5 + 1).contains(&cp) {
        return true;
    }
    if (0x1D6A8..0x1D6C0 + 1).contains(&cp) {
        return true;
    }
    if (0x1D6C2..0x1D6DA + 1).contains(&cp) {
        return true;
    }
    if (0x1D6DC..0x1D6FA + 1).contains(&cp) {
        return true;
    }
    if (0x1D6FC..0x1D714 + 1).contains(&cp) {
        return true;
    }
    if (0x1D716..0x1D734 + 1).contains(&cp) {
        return true;
    }
    if (0x1D736..0x1D74E + 1).contains(&cp) {
        return true;
    }
    if (0x1D750..0x1D76E + 1).contains(&cp) {
        return true;
    }
    if (0x1D770..0x1D788 + 1).contains(&cp) {
        return true;
    }
    if (0x1D78A..0x1D7A8 + 1).contains(&cp) {
        return true;
    }
    if (0x1D7AA..0x1D7C2 + 1).contains(&cp) {
        return true;
    }
    if (0x1D7C4..0x1D7CB + 1).contains(&cp) {
        return true;
    }
    if (0x1D7CE..0x1D7FF + 1).contains(&cp) {
        return true;
    }
    if (0x1EE00..0x1EE03 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE05..0x1EE1F + 1).contains(&cp) {
        return true;
    }
    if (0x1EE21..0x1EE22 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE24..0x1EE24 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE27..0x1EE27 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE29..0x1EE32 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE34..0x1EE37 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE39..0x1EE39 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE3B..0x1EE3B + 1).contains(&cp) {
        return true;
    }
    if (0x1EE42..0x1EE42 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE47..0x1EE47 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE49..0x1EE49 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE4B..0x1EE4B + 1).contains(&cp) {
        return true;
    }
    if (0x1EE4D..0x1EE4F + 1).contains(&cp) {
        return true;
    }
    if (0x1EE51..0x1EE52 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE54..0x1EE54 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE57..0x1EE57 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE59..0x1EE59 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE5B..0x1EE5B + 1).contains(&cp) {
        return true;
    }
    if (0x1EE5D..0x1EE5D + 1).contains(&cp) {
        return true;
    }
    if (0x1EE5F..0x1EE5F + 1).contains(&cp) {
        return true;
    }
    if (0x1EE61..0x1EE62 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE64..0x1EE64 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE67..0x1EE6A + 1).contains(&cp) {
        return true;
    }
    if (0x1EE6C..0x1EE72 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE74..0x1EE77 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE79..0x1EE7C + 1).contains(&cp) {
        return true;
    }
    if (0x1EE7E..0x1EE7E + 1).contains(&cp) {
        return true;
    }
    if (0x1EE80..0x1EE89 + 1).contains(&cp) {
        return true;
    }
    if (0x1EE8B..0x1EE9B + 1).contains(&cp) {
        return true;
    }
    if (0x1EEA1..0x1EEA3 + 1).contains(&cp) {
        return true;
    }
    if (0x1EEA5..0x1EEA9 + 1).contains(&cp) {
        return true;
    }
    if (0x1EEAB..0x1EEBB + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn hex(cp: u32) -> bool {
    if (0x0030..0x0039 + 1).contains(&cp) {
        return true;
    }
    if (0x0041..0x0046 + 1).contains(&cp) {
        return true;
    }
    if (0x0061..0x0066 + 1).contains(&cp) {
        return true;
    }
    if (0xFF10..0xFF19 + 1).contains(&cp) {
        return true;
    }
    if (0xFF21..0xFF26 + 1).contains(&cp) {
        return true;
    }
    if (0xFF41..0xFF46 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn ahex(cp: u32) -> bool {
    if (0x0030..0x0039 + 1).contains(&cp) {
        return true;
    }
    if (0x0041..0x0046 + 1).contains(&cp) {
        return true;
    }
    if (0x0061..0x0066 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn oalpha(cp: u32) -> bool {
    if (0x0345..0x0345 + 1).contains(&cp) {
        return true;
    }
    if (0x05B0..0x05BD + 1).contains(&cp) {
        return true;
    }
    if (0x05BF..0x05BF + 1).contains(&cp) {
        return true;
    }
    if (0x05C1..0x05C2 + 1).contains(&cp) {
        return true;
    }
    if (0x05C4..0x05C5 + 1).contains(&cp) {
        return true;
    }
    if (0x05C7..0x05C7 + 1).contains(&cp) {
        return true;
    }
    if (0x0610..0x061A + 1).contains(&cp) {
        return true;
    }
    if (0x064B..0x0657 + 1).contains(&cp) {
        return true;
    }
    if (0x0659..0x065F + 1).contains(&cp) {
        return true;
    }
    if (0x0670..0x0670 + 1).contains(&cp) {
        return true;
    }
    if (0x06D6..0x06DC + 1).contains(&cp) {
        return true;
    }
    if (0x06E1..0x06E4 + 1).contains(&cp) {
        return true;
    }
    if (0x06E7..0x06E8 + 1).contains(&cp) {
        return true;
    }
    if (0x06ED..0x06ED + 1).contains(&cp) {
        return true;
    }
    if (0x0711..0x0711 + 1).contains(&cp) {
        return true;
    }
    if (0x0730..0x073F + 1).contains(&cp) {
        return true;
    }
    if (0x07A6..0x07B0 + 1).contains(&cp) {
        return true;
    }
    if (0x0816..0x0817 + 1).contains(&cp) {
        return true;
    }
    if (0x081B..0x0823 + 1).contains(&cp) {
        return true;
    }
    if (0x0825..0x0827 + 1).contains(&cp) {
        return true;
    }
    if (0x0829..0x082C + 1).contains(&cp) {
        return true;
    }
    if (0x08D4..0x08DF + 1).contains(&cp) {
        return true;
    }
    if (0x08E3..0x08E9 + 1).contains(&cp) {
        return true;
    }
    if (0x08F0..0x0902 + 1).contains(&cp) {
        return true;
    }
    if (0x0903..0x0903 + 1).contains(&cp) {
        return true;
    }
    if (0x093A..0x093A + 1).contains(&cp) {
        return true;
    }
    if (0x093B..0x093B + 1).contains(&cp) {
        return true;
    }
    if (0x093E..0x0940 + 1).contains(&cp) {
        return true;
    }
    if (0x0941..0x0948 + 1).contains(&cp) {
        return true;
    }
    if (0x0949..0x094C + 1).contains(&cp) {
        return true;
    }
    if (0x094E..0x094F + 1).contains(&cp) {
        return true;
    }
    if (0x0955..0x0957 + 1).contains(&cp) {
        return true;
    }
    if (0x0962..0x0963 + 1).contains(&cp) {
        return true;
    }
    if (0x0981..0x0981 + 1).contains(&cp) {
        return true;
    }
    if (0x0982..0x0983 + 1).contains(&cp) {
        return true;
    }
    if (0x09BE..0x09C0 + 1).contains(&cp) {
        return true;
    }
    if (0x09C1..0x09C4 + 1).contains(&cp) {
        return true;
    }
    if (0x09C7..0x09C8 + 1).contains(&cp) {
        return true;
    }
    if (0x09CB..0x09CC + 1).contains(&cp) {
        return true;
    }
    if (0x09D7..0x09D7 + 1).contains(&cp) {
        return true;
    }
    if (0x09E2..0x09E3 + 1).contains(&cp) {
        return true;
    }
    if (0x0A01..0x0A02 + 1).contains(&cp) {
        return true;
    }
    if (0x0A03..0x0A03 + 1).contains(&cp) {
        return true;
    }
    if (0x0A3E..0x0A40 + 1).contains(&cp) {
        return true;
    }
    if (0x0A41..0x0A42 + 1).contains(&cp) {
        return true;
    }
    if (0x0A47..0x0A48 + 1).contains(&cp) {
        return true;
    }
    if (0x0A4B..0x0A4C + 1).contains(&cp) {
        return true;
    }
    if (0x0A51..0x0A51 + 1).contains(&cp) {
        return true;
    }
    if (0x0A70..0x0A71 + 1).contains(&cp) {
        return true;
    }
    if (0x0A75..0x0A75 + 1).contains(&cp) {
        return true;
    }
    if (0x0A81..0x0A82 + 1).contains(&cp) {
        return true;
    }
    if (0x0A83..0x0A83 + 1).contains(&cp) {
        return true;
    }
    if (0x0ABE..0x0AC0 + 1).contains(&cp) {
        return true;
    }
    if (0x0AC1..0x0AC5 + 1).contains(&cp) {
        return true;
    }
    if (0x0AC7..0x0AC8 + 1).contains(&cp) {
        return true;
    }
    if (0x0AC9..0x0AC9 + 1).contains(&cp) {
        return true;
    }
    if (0x0ACB..0x0ACC + 1).contains(&cp) {
        return true;
    }
    if (0x0AE2..0x0AE3 + 1).contains(&cp) {
        return true;
    }
    if (0x0AFA..0x0AFC + 1).contains(&cp) {
        return true;
    }
    if (0x0B01..0x0B01 + 1).contains(&cp) {
        return true;
    }
    if (0x0B02..0x0B03 + 1).contains(&cp) {
        return true;
    }
    if (0x0B3E..0x0B3E + 1).contains(&cp) {
        return true;
    }
    if (0x0B3F..0x0B3F + 1).contains(&cp) {
        return true;
    }
    if (0x0B40..0x0B40 + 1).contains(&cp) {
        return true;
    }
    if (0x0B41..0x0B44 + 1).contains(&cp) {
        return true;
    }
    if (0x0B47..0x0B48 + 1).contains(&cp) {
        return true;
    }
    if (0x0B4B..0x0B4C + 1).contains(&cp) {
        return true;
    }
    if (0x0B56..0x0B56 + 1).contains(&cp) {
        return true;
    }
    if (0x0B57..0x0B57 + 1).contains(&cp) {
        return true;
    }
    if (0x0B62..0x0B63 + 1).contains(&cp) {
        return true;
    }
    if (0x0B82..0x0B82 + 1).contains(&cp) {
        return true;
    }
    if (0x0BBE..0x0BBF + 1).contains(&cp) {
        return true;
    }
    if (0x0BC0..0x0BC0 + 1).contains(&cp) {
        return true;
    }
    if (0x0BC1..0x0BC2 + 1).contains(&cp) {
        return true;
    }
    if (0x0BC6..0x0BC8 + 1).contains(&cp) {
        return true;
    }
    if (0x0BCA..0x0BCC + 1).contains(&cp) {
        return true;
    }
    if (0x0BD7..0x0BD7 + 1).contains(&cp) {
        return true;
    }
    if (0x0C00..0x0C00 + 1).contains(&cp) {
        return true;
    }
    if (0x0C01..0x0C03 + 1).contains(&cp) {
        return true;
    }
    if (0x0C04..0x0C04 + 1).contains(&cp) {
        return true;
    }
    if (0x0C3E..0x0C40 + 1).contains(&cp) {
        return true;
    }
    if (0x0C41..0x0C44 + 1).contains(&cp) {
        return true;
    }
    if (0x0C46..0x0C48 + 1).contains(&cp) {
        return true;
    }
    if (0x0C4A..0x0C4C + 1).contains(&cp) {
        return true;
    }
    if (0x0C55..0x0C56 + 1).contains(&cp) {
        return true;
    }
    if (0x0C62..0x0C63 + 1).contains(&cp) {
        return true;
    }
    if (0x0C81..0x0C81 + 1).contains(&cp) {
        return true;
    }
    if (0x0C82..0x0C83 + 1).contains(&cp) {
        return true;
    }
    if (0x0CBE..0x0CBE + 1).contains(&cp) {
        return true;
    }
    if (0x0CBF..0x0CBF + 1).contains(&cp) {
        return true;
    }
    if (0x0CC0..0x0CC4 + 1).contains(&cp) {
        return true;
    }
    if (0x0CC6..0x0CC6 + 1).contains(&cp) {
        return true;
    }
    if (0x0CC7..0x0CC8 + 1).contains(&cp) {
        return true;
    }
    if (0x0CCA..0x0CCB + 1).contains(&cp) {
        return true;
    }
    if (0x0CCC..0x0CCC + 1).contains(&cp) {
        return true;
    }
    if (0x0CD5..0x0CD6 + 1).contains(&cp) {
        return true;
    }
    if (0x0CE2..0x0CE3 + 1).contains(&cp) {
        return true;
    }
    if (0x0CF3..0x0CF3 + 1).contains(&cp) {
        return true;
    }
    if (0x0D00..0x0D01 + 1).contains(&cp) {
        return true;
    }
    if (0x0D02..0x0D03 + 1).contains(&cp) {
        return true;
    }
    if (0x0D3E..0x0D40 + 1).contains(&cp) {
        return true;
    }
    if (0x0D41..0x0D44 + 1).contains(&cp) {
        return true;
    }
    if (0x0D46..0x0D48 + 1).contains(&cp) {
        return true;
    }
    if (0x0D4A..0x0D4C + 1).contains(&cp) {
        return true;
    }
    if (0x0D57..0x0D57 + 1).contains(&cp) {
        return true;
    }
    if (0x0D62..0x0D63 + 1).contains(&cp) {
        return true;
    }
    if (0x0D81..0x0D81 + 1).contains(&cp) {
        return true;
    }
    if (0x0D82..0x0D83 + 1).contains(&cp) {
        return true;
    }
    if (0x0DCF..0x0DD1 + 1).contains(&cp) {
        return true;
    }
    if (0x0DD2..0x0DD4 + 1).contains(&cp) {
        return true;
    }
    if (0x0DD6..0x0DD6 + 1).contains(&cp) {
        return true;
    }
    if (0x0DD8..0x0DDF + 1).contains(&cp) {
        return true;
    }
    if (0x0DF2..0x0DF3 + 1).contains(&cp) {
        return true;
    }
    if (0x0E31..0x0E31 + 1).contains(&cp) {
        return true;
    }
    if (0x0E34..0x0E3A + 1).contains(&cp) {
        return true;
    }
    if (0x0E4D..0x0E4D + 1).contains(&cp) {
        return true;
    }
    if (0x0EB1..0x0EB1 + 1).contains(&cp) {
        return true;
    }
    if (0x0EB4..0x0EB9 + 1).contains(&cp) {
        return true;
    }
    if (0x0EBB..0x0EBC + 1).contains(&cp) {
        return true;
    }
    if (0x0ECD..0x0ECD + 1).contains(&cp) {
        return true;
    }
    if (0x0F71..0x0F7E + 1).contains(&cp) {
        return true;
    }
    if (0x0F7F..0x0F7F + 1).contains(&cp) {
        return true;
    }
    if (0x0F80..0x0F83 + 1).contains(&cp) {
        return true;
    }
    if (0x0F8D..0x0F97 + 1).contains(&cp) {
        return true;
    }
    if (0x0F99..0x0FBC + 1).contains(&cp) {
        return true;
    }
    if (0x102B..0x102C + 1).contains(&cp) {
        return true;
    }
    if (0x102D..0x1030 + 1).contains(&cp) {
        return true;
    }
    if (0x1031..0x1031 + 1).contains(&cp) {
        return true;
    }
    if (0x1032..0x1036 + 1).contains(&cp) {
        return true;
    }
    if (0x1038..0x1038 + 1).contains(&cp) {
        return true;
    }
    if (0x103B..0x103C + 1).contains(&cp) {
        return true;
    }
    if (0x103D..0x103E + 1).contains(&cp) {
        return true;
    }
    if (0x1056..0x1057 + 1).contains(&cp) {
        return true;
    }
    if (0x1058..0x1059 + 1).contains(&cp) {
        return true;
    }
    if (0x105E..0x1060 + 1).contains(&cp) {
        return true;
    }
    if (0x1062..0x1064 + 1).contains(&cp) {
        return true;
    }
    if (0x1067..0x106D + 1).contains(&cp) {
        return true;
    }
    if (0x1071..0x1074 + 1).contains(&cp) {
        return true;
    }
    if (0x1082..0x1082 + 1).contains(&cp) {
        return true;
    }
    if (0x1083..0x1084 + 1).contains(&cp) {
        return true;
    }
    if (0x1085..0x1086 + 1).contains(&cp) {
        return true;
    }
    if (0x1087..0x108C + 1).contains(&cp) {
        return true;
    }
    if (0x108D..0x108D + 1).contains(&cp) {
        return true;
    }
    if (0x108F..0x108F + 1).contains(&cp) {
        return true;
    }
    if (0x109A..0x109C + 1).contains(&cp) {
        return true;
    }
    if (0x109D..0x109D + 1).contains(&cp) {
        return true;
    }
    if (0x1712..0x1713 + 1).contains(&cp) {
        return true;
    }
    if (0x1732..0x1733 + 1).contains(&cp) {
        return true;
    }
    if (0x1752..0x1753 + 1).contains(&cp) {
        return true;
    }
    if (0x1772..0x1773 + 1).contains(&cp) {
        return true;
    }
    if (0x17B6..0x17B6 + 1).contains(&cp) {
        return true;
    }
    if (0x17B7..0x17BD + 1).contains(&cp) {
        return true;
    }
    if (0x17BE..0x17C5 + 1).contains(&cp) {
        return true;
    }
    if (0x17C6..0x17C6 + 1).contains(&cp) {
        return true;
    }
    if (0x17C7..0x17C8 + 1).contains(&cp) {
        return true;
    }
    if (0x1885..0x1886 + 1).contains(&cp) {
        return true;
    }
    if (0x18A9..0x18A9 + 1).contains(&cp) {
        return true;
    }
    if (0x1920..0x1922 + 1).contains(&cp) {
        return true;
    }
    if (0x1923..0x1926 + 1).contains(&cp) {
        return true;
    }
    if (0x1927..0x1928 + 1).contains(&cp) {
        return true;
    }
    if (0x1929..0x192B + 1).contains(&cp) {
        return true;
    }
    if (0x1930..0x1931 + 1).contains(&cp) {
        return true;
    }
    if (0x1932..0x1932 + 1).contains(&cp) {
        return true;
    }
    if (0x1933..0x1938 + 1).contains(&cp) {
        return true;
    }
    if (0x1A17..0x1A18 + 1).contains(&cp) {
        return true;
    }
    if (0x1A19..0x1A1A + 1).contains(&cp) {
        return true;
    }
    if (0x1A1B..0x1A1B + 1).contains(&cp) {
        return true;
    }
    if (0x1A55..0x1A55 + 1).contains(&cp) {
        return true;
    }
    if (0x1A56..0x1A56 + 1).contains(&cp) {
        return true;
    }
    if (0x1A57..0x1A57 + 1).contains(&cp) {
        return true;
    }
    if (0x1A58..0x1A5E + 1).contains(&cp) {
        return true;
    }
    if (0x1A61..0x1A61 + 1).contains(&cp) {
        return true;
    }
    if (0x1A62..0x1A62 + 1).contains(&cp) {
        return true;
    }
    if (0x1A63..0x1A64 + 1).contains(&cp) {
        return true;
    }
    if (0x1A65..0x1A6C + 1).contains(&cp) {
        return true;
    }
    if (0x1A6D..0x1A72 + 1).contains(&cp) {
        return true;
    }
    if (0x1A73..0x1A74 + 1).contains(&cp) {
        return true;
    }
    if (0x1ABF..0x1AC0 + 1).contains(&cp) {
        return true;
    }
    if (0x1ACC..0x1ACE + 1).contains(&cp) {
        return true;
    }
    if (0x1B00..0x1B03 + 1).contains(&cp) {
        return true;
    }
    if (0x1B04..0x1B04 + 1).contains(&cp) {
        return true;
    }
    if (0x1B35..0x1B35 + 1).contains(&cp) {
        return true;
    }
    if (0x1B36..0x1B3A + 1).contains(&cp) {
        return true;
    }
    if (0x1B3B..0x1B3B + 1).contains(&cp) {
        return true;
    }
    if (0x1B3C..0x1B3C + 1).contains(&cp) {
        return true;
    }
    if (0x1B3D..0x1B41 + 1).contains(&cp) {
        return true;
    }
    if (0x1B42..0x1B42 + 1).contains(&cp) {
        return true;
    }
    if (0x1B43..0x1B43 + 1).contains(&cp) {
        return true;
    }
    if (0x1B80..0x1B81 + 1).contains(&cp) {
        return true;
    }
    if (0x1B82..0x1B82 + 1).contains(&cp) {
        return true;
    }
    if (0x1BA1..0x1BA1 + 1).contains(&cp) {
        return true;
    }
    if (0x1BA2..0x1BA5 + 1).contains(&cp) {
        return true;
    }
    if (0x1BA6..0x1BA7 + 1).contains(&cp) {
        return true;
    }
    if (0x1BA8..0x1BA9 + 1).contains(&cp) {
        return true;
    }
    if (0x1BAC..0x1BAD + 1).contains(&cp) {
        return true;
    }
    if (0x1BE7..0x1BE7 + 1).contains(&cp) {
        return true;
    }
    if (0x1BE8..0x1BE9 + 1).contains(&cp) {
        return true;
    }
    if (0x1BEA..0x1BEC + 1).contains(&cp) {
        return true;
    }
    if (0x1BED..0x1BED + 1).contains(&cp) {
        return true;
    }
    if (0x1BEE..0x1BEE + 1).contains(&cp) {
        return true;
    }
    if (0x1BEF..0x1BF1 + 1).contains(&cp) {
        return true;
    }
    if (0x1C24..0x1C2B + 1).contains(&cp) {
        return true;
    }
    if (0x1C2C..0x1C33 + 1).contains(&cp) {
        return true;
    }
    if (0x1C34..0x1C35 + 1).contains(&cp) {
        return true;
    }
    if (0x1C36..0x1C36 + 1).contains(&cp) {
        return true;
    }
    if (0x1DE7..0x1DF4 + 1).contains(&cp) {
        return true;
    }
    if (0x24B6..0x24E9 + 1).contains(&cp) {
        return true;
    }
    if (0x2DE0..0x2DFF + 1).contains(&cp) {
        return true;
    }
    if (0xA674..0xA67B + 1).contains(&cp) {
        return true;
    }
    if (0xA69E..0xA69F + 1).contains(&cp) {
        return true;
    }
    if (0xA802..0xA802 + 1).contains(&cp) {
        return true;
    }
    if (0xA80B..0xA80B + 1).contains(&cp) {
        return true;
    }
    if (0xA823..0xA824 + 1).contains(&cp) {
        return true;
    }
    if (0xA825..0xA826 + 1).contains(&cp) {
        return true;
    }
    if (0xA827..0xA827 + 1).contains(&cp) {
        return true;
    }
    if (0xA880..0xA881 + 1).contains(&cp) {
        return true;
    }
    if (0xA8B4..0xA8C3 + 1).contains(&cp) {
        return true;
    }
    if (0xA8C5..0xA8C5 + 1).contains(&cp) {
        return true;
    }
    if (0xA8FF..0xA8FF + 1).contains(&cp) {
        return true;
    }
    if (0xA926..0xA92A + 1).contains(&cp) {
        return true;
    }
    if (0xA947..0xA951 + 1).contains(&cp) {
        return true;
    }
    if (0xA952..0xA952 + 1).contains(&cp) {
        return true;
    }
    if (0xA980..0xA982 + 1).contains(&cp) {
        return true;
    }
    if (0xA983..0xA983 + 1).contains(&cp) {
        return true;
    }
    if (0xA9B4..0xA9B5 + 1).contains(&cp) {
        return true;
    }
    if (0xA9B6..0xA9B9 + 1).contains(&cp) {
        return true;
    }
    if (0xA9BA..0xA9BB + 1).contains(&cp) {
        return true;
    }
    if (0xA9BC..0xA9BD + 1).contains(&cp) {
        return true;
    }
    if (0xA9BE..0xA9BF + 1).contains(&cp) {
        return true;
    }
    if (0xA9E5..0xA9E5 + 1).contains(&cp) {
        return true;
    }
    if (0xAA29..0xAA2E + 1).contains(&cp) {
        return true;
    }
    if (0xAA2F..0xAA30 + 1).contains(&cp) {
        return true;
    }
    if (0xAA31..0xAA32 + 1).contains(&cp) {
        return true;
    }
    if (0xAA33..0xAA34 + 1).contains(&cp) {
        return true;
    }
    if (0xAA35..0xAA36 + 1).contains(&cp) {
        return true;
    }
    if (0xAA43..0xAA43 + 1).contains(&cp) {
        return true;
    }
    if (0xAA4C..0xAA4C + 1).contains(&cp) {
        return true;
    }
    if (0xAA4D..0xAA4D + 1).contains(&cp) {
        return true;
    }
    if (0xAA7B..0xAA7B + 1).contains(&cp) {
        return true;
    }
    if (0xAA7C..0xAA7C + 1).contains(&cp) {
        return true;
    }
    if (0xAA7D..0xAA7D + 1).contains(&cp) {
        return true;
    }
    if (0xAAB0..0xAAB0 + 1).contains(&cp) {
        return true;
    }
    if (0xAAB2..0xAAB4 + 1).contains(&cp) {
        return true;
    }
    if (0xAAB7..0xAAB8 + 1).contains(&cp) {
        return true;
    }
    if (0xAABE..0xAABE + 1).contains(&cp) {
        return true;
    }
    if (0xAAEB..0xAAEB + 1).contains(&cp) {
        return true;
    }
    if (0xAAEC..0xAAED + 1).contains(&cp) {
        return true;
    }
    if (0xAAEE..0xAAEF + 1).contains(&cp) {
        return true;
    }
    if (0xAAF5..0xAAF5 + 1).contains(&cp) {
        return true;
    }
    if (0xABE3..0xABE4 + 1).contains(&cp) {
        return true;
    }
    if (0xABE5..0xABE5 + 1).contains(&cp) {
        return true;
    }
    if (0xABE6..0xABE7 + 1).contains(&cp) {
        return true;
    }
    if (0xABE8..0xABE8 + 1).contains(&cp) {
        return true;
    }
    if (0xABE9..0xABEA + 1).contains(&cp) {
        return true;
    }
    if (0xFB1E..0xFB1E + 1).contains(&cp) {
        return true;
    }
    if (0x10376..0x1037A + 1).contains(&cp) {
        return true;
    }
    if (0x10A01..0x10A03 + 1).contains(&cp) {
        return true;
    }
    if (0x10A05..0x10A06 + 1).contains(&cp) {
        return true;
    }
    if (0x10A0C..0x10A0F + 1).contains(&cp) {
        return true;
    }
    if (0x10D24..0x10D27 + 1).contains(&cp) {
        return true;
    }
    if (0x10EAB..0x10EAC + 1).contains(&cp) {
        return true;
    }
    if (0x11000..0x11000 + 1).contains(&cp) {
        return true;
    }
    if (0x11001..0x11001 + 1).contains(&cp) {
        return true;
    }
    if (0x11002..0x11002 + 1).contains(&cp) {
        return true;
    }
    if (0x11038..0x11045 + 1).contains(&cp) {
        return true;
    }
    if (0x11073..0x11074 + 1).contains(&cp) {
        return true;
    }
    if (0x11080..0x11081 + 1).contains(&cp) {
        return true;
    }
    if (0x11082..0x11082 + 1).contains(&cp) {
        return true;
    }
    if (0x110B0..0x110B2 + 1).contains(&cp) {
        return true;
    }
    if (0x110B3..0x110B6 + 1).contains(&cp) {
        return true;
    }
    if (0x110B7..0x110B8 + 1).contains(&cp) {
        return true;
    }
    if (0x110C2..0x110C2 + 1).contains(&cp) {
        return true;
    }
    if (0x11100..0x11102 + 1).contains(&cp) {
        return true;
    }
    if (0x11127..0x1112B + 1).contains(&cp) {
        return true;
    }
    if (0x1112C..0x1112C + 1).contains(&cp) {
        return true;
    }
    if (0x1112D..0x11132 + 1).contains(&cp) {
        return true;
    }
    if (0x11145..0x11146 + 1).contains(&cp) {
        return true;
    }
    if (0x11180..0x11181 + 1).contains(&cp) {
        return true;
    }
    if (0x11182..0x11182 + 1).contains(&cp) {
        return true;
    }
    if (0x111B3..0x111B5 + 1).contains(&cp) {
        return true;
    }
    if (0x111B6..0x111BE + 1).contains(&cp) {
        return true;
    }
    if (0x111BF..0x111BF + 1).contains(&cp) {
        return true;
    }
    if (0x111CE..0x111CE + 1).contains(&cp) {
        return true;
    }
    if (0x111CF..0x111CF + 1).contains(&cp) {
        return true;
    }
    if (0x1122C..0x1122E + 1).contains(&cp) {
        return true;
    }
    if (0x1122F..0x11231 + 1).contains(&cp) {
        return true;
    }
    if (0x11232..0x11233 + 1).contains(&cp) {
        return true;
    }
    if (0x11234..0x11234 + 1).contains(&cp) {
        return true;
    }
    if (0x11237..0x11237 + 1).contains(&cp) {
        return true;
    }
    if (0x1123E..0x1123E + 1).contains(&cp) {
        return true;
    }
    if (0x11241..0x11241 + 1).contains(&cp) {
        return true;
    }
    if (0x112DF..0x112DF + 1).contains(&cp) {
        return true;
    }
    if (0x112E0..0x112E2 + 1).contains(&cp) {
        return true;
    }
    if (0x112E3..0x112E8 + 1).contains(&cp) {
        return true;
    }
    if (0x11300..0x11301 + 1).contains(&cp) {
        return true;
    }
    if (0x11302..0x11303 + 1).contains(&cp) {
        return true;
    }
    if (0x1133E..0x1133F + 1).contains(&cp) {
        return true;
    }
    if (0x11340..0x11340 + 1).contains(&cp) {
        return true;
    }
    if (0x11341..0x11344 + 1).contains(&cp) {
        return true;
    }
    if (0x11347..0x11348 + 1).contains(&cp) {
        return true;
    }
    if (0x1134B..0x1134C + 1).contains(&cp) {
        return true;
    }
    if (0x11357..0x11357 + 1).contains(&cp) {
        return true;
    }
    if (0x11362..0x11363 + 1).contains(&cp) {
        return true;
    }
    if (0x11435..0x11437 + 1).contains(&cp) {
        return true;
    }
    if (0x11438..0x1143F + 1).contains(&cp) {
        return true;
    }
    if (0x11440..0x11441 + 1).contains(&cp) {
        return true;
    }
    if (0x11443..0x11444 + 1).contains(&cp) {
        return true;
    }
    if (0x11445..0x11445 + 1).contains(&cp) {
        return true;
    }
    if (0x114B0..0x114B2 + 1).contains(&cp) {
        return true;
    }
    if (0x114B3..0x114B8 + 1).contains(&cp) {
        return true;
    }
    if (0x114B9..0x114B9 + 1).contains(&cp) {
        return true;
    }
    if (0x114BA..0x114BA + 1).contains(&cp) {
        return true;
    }
    if (0x114BB..0x114BE + 1).contains(&cp) {
        return true;
    }
    if (0x114BF..0x114C0 + 1).contains(&cp) {
        return true;
    }
    if (0x114C1..0x114C1 + 1).contains(&cp) {
        return true;
    }
    if (0x115AF..0x115B1 + 1).contains(&cp) {
        return true;
    }
    if (0x115B2..0x115B5 + 1).contains(&cp) {
        return true;
    }
    if (0x115B8..0x115BB + 1).contains(&cp) {
        return true;
    }
    if (0x115BC..0x115BD + 1).contains(&cp) {
        return true;
    }
    if (0x115BE..0x115BE + 1).contains(&cp) {
        return true;
    }
    if (0x115DC..0x115DD + 1).contains(&cp) {
        return true;
    }
    if (0x11630..0x11632 + 1).contains(&cp) {
        return true;
    }
    if (0x11633..0x1163A + 1).contains(&cp) {
        return true;
    }
    if (0x1163B..0x1163C + 1).contains(&cp) {
        return true;
    }
    if (0x1163D..0x1163D + 1).contains(&cp) {
        return true;
    }
    if (0x1163E..0x1163E + 1).contains(&cp) {
        return true;
    }
    if (0x11640..0x11640 + 1).contains(&cp) {
        return true;
    }
    if (0x116AB..0x116AB + 1).contains(&cp) {
        return true;
    }
    if (0x116AC..0x116AC + 1).contains(&cp) {
        return true;
    }
    if (0x116AD..0x116AD + 1).contains(&cp) {
        return true;
    }
    if (0x116AE..0x116AF + 1).contains(&cp) {
        return true;
    }
    if (0x116B0..0x116B5 + 1).contains(&cp) {
        return true;
    }
    if (0x1171D..0x1171F + 1).contains(&cp) {
        return true;
    }
    if (0x11720..0x11721 + 1).contains(&cp) {
        return true;
    }
    if (0x11722..0x11725 + 1).contains(&cp) {
        return true;
    }
    if (0x11726..0x11726 + 1).contains(&cp) {
        return true;
    }
    if (0x11727..0x1172A + 1).contains(&cp) {
        return true;
    }
    if (0x1182C..0x1182E + 1).contains(&cp) {
        return true;
    }
    if (0x1182F..0x11837 + 1).contains(&cp) {
        return true;
    }
    if (0x11838..0x11838 + 1).contains(&cp) {
        return true;
    }
    if (0x11930..0x11935 + 1).contains(&cp) {
        return true;
    }
    if (0x11937..0x11938 + 1).contains(&cp) {
        return true;
    }
    if (0x1193B..0x1193C + 1).contains(&cp) {
        return true;
    }
    if (0x11940..0x11940 + 1).contains(&cp) {
        return true;
    }
    if (0x11942..0x11942 + 1).contains(&cp) {
        return true;
    }
    if (0x119D1..0x119D3 + 1).contains(&cp) {
        return true;
    }
    if (0x119D4..0x119D7 + 1).contains(&cp) {
        return true;
    }
    if (0x119DA..0x119DB + 1).contains(&cp) {
        return true;
    }
    if (0x119DC..0x119DF + 1).contains(&cp) {
        return true;
    }
    if (0x119E4..0x119E4 + 1).contains(&cp) {
        return true;
    }
    if (0x11A01..0x11A0A + 1).contains(&cp) {
        return true;
    }
    if (0x11A35..0x11A38 + 1).contains(&cp) {
        return true;
    }
    if (0x11A39..0x11A39 + 1).contains(&cp) {
        return true;
    }
    if (0x11A3B..0x11A3E + 1).contains(&cp) {
        return true;
    }
    if (0x11A51..0x11A56 + 1).contains(&cp) {
        return true;
    }
    if (0x11A57..0x11A58 + 1).contains(&cp) {
        return true;
    }
    if (0x11A59..0x11A5B + 1).contains(&cp) {
        return true;
    }
    if (0x11A8A..0x11A96 + 1).contains(&cp) {
        return true;
    }
    if (0x11A97..0x11A97 + 1).contains(&cp) {
        return true;
    }
    if (0x11C2F..0x11C2F + 1).contains(&cp) {
        return true;
    }
    if (0x11C30..0x11C36 + 1).contains(&cp) {
        return true;
    }
    if (0x11C38..0x11C3D + 1).contains(&cp) {
        return true;
    }
    if (0x11C3E..0x11C3E + 1).contains(&cp) {
        return true;
    }
    if (0x11C92..0x11CA7 + 1).contains(&cp) {
        return true;
    }
    if (0x11CA9..0x11CA9 + 1).contains(&cp) {
        return true;
    }
    if (0x11CAA..0x11CB0 + 1).contains(&cp) {
        return true;
    }
    if (0x11CB1..0x11CB1 + 1).contains(&cp) {
        return true;
    }
    if (0x11CB2..0x11CB3 + 1).contains(&cp) {
        return true;
    }
    if (0x11CB4..0x11CB4 + 1).contains(&cp) {
        return true;
    }
    if (0x11CB5..0x11CB6 + 1).contains(&cp) {
        return true;
    }
    if (0x11D31..0x11D36 + 1).contains(&cp) {
        return true;
    }
    if (0x11D3A..0x11D3A + 1).contains(&cp) {
        return true;
    }
    if (0x11D3C..0x11D3D + 1).contains(&cp) {
        return true;
    }
    if (0x11D3F..0x11D41 + 1).contains(&cp) {
        return true;
    }
    if (0x11D43..0x11D43 + 1).contains(&cp) {
        return true;
    }
    if (0x11D47..0x11D47 + 1).contains(&cp) {
        return true;
    }
    if (0x11D8A..0x11D8E + 1).contains(&cp) {
        return true;
    }
    if (0x11D90..0x11D91 + 1).contains(&cp) {
        return true;
    }
    if (0x11D93..0x11D94 + 1).contains(&cp) {
        return true;
    }
    if (0x11D95..0x11D95 + 1).contains(&cp) {
        return true;
    }
    if (0x11D96..0x11D96 + 1).contains(&cp) {
        return true;
    }
    if (0x11EF3..0x11EF4 + 1).contains(&cp) {
        return true;
    }
    if (0x11EF5..0x11EF6 + 1).contains(&cp) {
        return true;
    }
    if (0x11F00..0x11F01 + 1).contains(&cp) {
        return true;
    }
    if (0x11F03..0x11F03 + 1).contains(&cp) {
        return true;
    }
    if (0x11F34..0x11F35 + 1).contains(&cp) {
        return true;
    }
    if (0x11F36..0x11F3A + 1).contains(&cp) {
        return true;
    }
    if (0x11F3E..0x11F3F + 1).contains(&cp) {
        return true;
    }
    if (0x11F40..0x11F40 + 1).contains(&cp) {
        return true;
    }
    if (0x16F4F..0x16F4F + 1).contains(&cp) {
        return true;
    }
    if (0x16F51..0x16F87 + 1).contains(&cp) {
        return true;
    }
    if (0x16F8F..0x16F92 + 1).contains(&cp) {
        return true;
    }
    if (0x16FF0..0x16FF1 + 1).contains(&cp) {
        return true;
    }
    if (0x1BC9E..0x1BC9E + 1).contains(&cp) {
        return true;
    }
    if (0x1E000..0x1E006 + 1).contains(&cp) {
        return true;
    }
    if (0x1E008..0x1E018 + 1).contains(&cp) {
        return true;
    }
    if (0x1E01B..0x1E021 + 1).contains(&cp) {
        return true;
    }
    if (0x1E023..0x1E024 + 1).contains(&cp) {
        return true;
    }
    if (0x1E026..0x1E02A + 1).contains(&cp) {
        return true;
    }
    if (0x1E08F..0x1E08F + 1).contains(&cp) {
        return true;
    }
    if (0x1E947..0x1E947 + 1).contains(&cp) {
        return true;
    }
    if (0x1F130..0x1F149 + 1).contains(&cp) {
        return true;
    }
    if (0x1F150..0x1F169 + 1).contains(&cp) {
        return true;
    }
    if (0x1F170..0x1F189 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn ideo(cp: u32) -> bool {
    if (0x3006..0x3006 + 1).contains(&cp) {
        return true;
    }
    if (0x3007..0x3007 + 1).contains(&cp) {
        return true;
    }
    if (0x3021..0x3029 + 1).contains(&cp) {
        return true;
    }
    if (0x3038..0x303A + 1).contains(&cp) {
        return true;
    }
    if (0x3400..0x4DBF + 1).contains(&cp) {
        return true;
    }
    if (0x4E00..0x9FFF + 1).contains(&cp) {
        return true;
    }
    if (0xF900..0xFA6D + 1).contains(&cp) {
        return true;
    }
    if (0xFA70..0xFAD9 + 1).contains(&cp) {
        return true;
    }
    if (0x16FE4..0x16FE4 + 1).contains(&cp) {
        return true;
    }
    if (0x17000..0x187F7 + 1).contains(&cp) {
        return true;
    }
    if (0x18800..0x18CD5 + 1).contains(&cp) {
        return true;
    }
    if (0x18D00..0x18D08 + 1).contains(&cp) {
        return true;
    }
    if (0x1B170..0x1B2FB + 1).contains(&cp) {
        return true;
    }
    if (0x20000..0x2A6DF + 1).contains(&cp) {
        return true;
    }
    if (0x2A700..0x2B739 + 1).contains(&cp) {
        return true;
    }
    if (0x2B740..0x2B81D + 1).contains(&cp) {
        return true;
    }
    if (0x2B820..0x2CEA1 + 1).contains(&cp) {
        return true;
    }
    if (0x2CEB0..0x2EBE0 + 1).contains(&cp) {
        return true;
    }
    if (0x2F800..0x2FA1D + 1).contains(&cp) {
        return true;
    }
    if (0x30000..0x3134A + 1).contains(&cp) {
        return true;
    }
    if (0x31350..0x323AF + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn dia(cp: u32) -> bool {
    if (0x005E..0x005E + 1).contains(&cp) {
        return true;
    }
    if (0x0060..0x0060 + 1).contains(&cp) {
        return true;
    }
    if (0x00A8..0x00A8 + 1).contains(&cp) {
        return true;
    }
    if (0x00AF..0x00AF + 1).contains(&cp) {
        return true;
    }
    if (0x00B4..0x00B4 + 1).contains(&cp) {
        return true;
    }
    if (0x00B7..0x00B7 + 1).contains(&cp) {
        return true;
    }
    if (0x00B8..0x00B8 + 1).contains(&cp) {
        return true;
    }
    if (0x02B0..0x02C1 + 1).contains(&cp) {
        return true;
    }
    if (0x02C2..0x02C5 + 1).contains(&cp) {
        return true;
    }
    if (0x02C6..0x02D1 + 1).contains(&cp) {
        return true;
    }
    if (0x02D2..0x02DF + 1).contains(&cp) {
        return true;
    }
    if (0x02E0..0x02E4 + 1).contains(&cp) {
        return true;
    }
    if (0x02E5..0x02EB + 1).contains(&cp) {
        return true;
    }
    if (0x02EC..0x02EC + 1).contains(&cp) {
        return true;
    }
    if (0x02ED..0x02ED + 1).contains(&cp) {
        return true;
    }
    if (0x02EE..0x02EE + 1).contains(&cp) {
        return true;
    }
    if (0x02EF..0x02FF + 1).contains(&cp) {
        return true;
    }
    if (0x0300..0x034E + 1).contains(&cp) {
        return true;
    }
    if (0x0350..0x0357 + 1).contains(&cp) {
        return true;
    }
    if (0x035D..0x0362 + 1).contains(&cp) {
        return true;
    }
    if (0x0374..0x0374 + 1).contains(&cp) {
        return true;
    }
    if (0x0375..0x0375 + 1).contains(&cp) {
        return true;
    }
    if (0x037A..0x037A + 1).contains(&cp) {
        return true;
    }
    if (0x0384..0x0385 + 1).contains(&cp) {
        return true;
    }
    if (0x0483..0x0487 + 1).contains(&cp) {
        return true;
    }
    if (0x0559..0x0559 + 1).contains(&cp) {
        return true;
    }
    if (0x0591..0x05A1 + 1).contains(&cp) {
        return true;
    }
    if (0x05A3..0x05BD + 1).contains(&cp) {
        return true;
    }
    if (0x05BF..0x05BF + 1).contains(&cp) {
        return true;
    }
    if (0x05C1..0x05C2 + 1).contains(&cp) {
        return true;
    }
    if (0x05C4..0x05C4 + 1).contains(&cp) {
        return true;
    }
    if (0x064B..0x0652 + 1).contains(&cp) {
        return true;
    }
    if (0x0657..0x0658 + 1).contains(&cp) {
        return true;
    }
    if (0x06DF..0x06E0 + 1).contains(&cp) {
        return true;
    }
    if (0x06E5..0x06E6 + 1).contains(&cp) {
        return true;
    }
    if (0x06EA..0x06EC + 1).contains(&cp) {
        return true;
    }
    if (0x0730..0x074A + 1).contains(&cp) {
        return true;
    }
    if (0x07A6..0x07B0 + 1).contains(&cp) {
        return true;
    }
    if (0x07EB..0x07F3 + 1).contains(&cp) {
        return true;
    }
    if (0x07F4..0x07F5 + 1).contains(&cp) {
        return true;
    }
    if (0x0818..0x0819 + 1).contains(&cp) {
        return true;
    }
    if (0x0898..0x089F + 1).contains(&cp) {
        return true;
    }
    if (0x08C9..0x08C9 + 1).contains(&cp) {
        return true;
    }
    if (0x08CA..0x08D2 + 1).contains(&cp) {
        return true;
    }
    if (0x08E3..0x08FE + 1).contains(&cp) {
        return true;
    }
    if (0x093C..0x093C + 1).contains(&cp) {
        return true;
    }
    if (0x094D..0x094D + 1).contains(&cp) {
        return true;
    }
    if (0x0951..0x0954 + 1).contains(&cp) {
        return true;
    }
    if (0x0971..0x0971 + 1).contains(&cp) {
        return true;
    }
    if (0x09BC..0x09BC + 1).contains(&cp) {
        return true;
    }
    if (0x09CD..0x09CD + 1).contains(&cp) {
        return true;
    }
    if (0x0A3C..0x0A3C + 1).contains(&cp) {
        return true;
    }
    if (0x0A4D..0x0A4D + 1).contains(&cp) {
        return true;
    }
    if (0x0ABC..0x0ABC + 1).contains(&cp) {
        return true;
    }
    if (0x0ACD..0x0ACD + 1).contains(&cp) {
        return true;
    }
    if (0x0AFD..0x0AFF + 1).contains(&cp) {
        return true;
    }
    if (0x0B3C..0x0B3C + 1).contains(&cp) {
        return true;
    }
    if (0x0B4D..0x0B4D + 1).contains(&cp) {
        return true;
    }
    if (0x0B55..0x0B55 + 1).contains(&cp) {
        return true;
    }
    if (0x0BCD..0x0BCD + 1).contains(&cp) {
        return true;
    }
    if (0x0C3C..0x0C3C + 1).contains(&cp) {
        return true;
    }
    if (0x0C4D..0x0C4D + 1).contains(&cp) {
        return true;
    }
    if (0x0CBC..0x0CBC + 1).contains(&cp) {
        return true;
    }
    if (0x0CCD..0x0CCD + 1).contains(&cp) {
        return true;
    }
    if (0x0D3B..0x0D3C + 1).contains(&cp) {
        return true;
    }
    if (0x0D4D..0x0D4D + 1).contains(&cp) {
        return true;
    }
    if (0x0DCA..0x0DCA + 1).contains(&cp) {
        return true;
    }
    if (0x0E47..0x0E4C + 1).contains(&cp) {
        return true;
    }
    if (0x0E4E..0x0E4E + 1).contains(&cp) {
        return true;
    }
    if (0x0EBA..0x0EBA + 1).contains(&cp) {
        return true;
    }
    if (0x0EC8..0x0ECC + 1).contains(&cp) {
        return true;
    }
    if (0x0F18..0x0F19 + 1).contains(&cp) {
        return true;
    }
    if (0x0F35..0x0F35 + 1).contains(&cp) {
        return true;
    }
    if (0x0F37..0x0F37 + 1).contains(&cp) {
        return true;
    }
    if (0x0F39..0x0F39 + 1).contains(&cp) {
        return true;
    }
    if (0x0F3E..0x0F3F + 1).contains(&cp) {
        return true;
    }
    if (0x0F82..0x0F84 + 1).contains(&cp) {
        return true;
    }
    if (0x0F86..0x0F87 + 1).contains(&cp) {
        return true;
    }
    if (0x0FC6..0x0FC6 + 1).contains(&cp) {
        return true;
    }
    if (0x1037..0x1037 + 1).contains(&cp) {
        return true;
    }
    if (0x1039..0x103A + 1).contains(&cp) {
        return true;
    }
    if (0x1063..0x1064 + 1).contains(&cp) {
        return true;
    }
    if (0x1069..0x106D + 1).contains(&cp) {
        return true;
    }
    if (0x1087..0x108C + 1).contains(&cp) {
        return true;
    }
    if (0x108D..0x108D + 1).contains(&cp) {
        return true;
    }
    if (0x108F..0x108F + 1).contains(&cp) {
        return true;
    }
    if (0x109A..0x109B + 1).contains(&cp) {
        return true;
    }
    if (0x135D..0x135F + 1).contains(&cp) {
        return true;
    }
    if (0x1714..0x1714 + 1).contains(&cp) {
        return true;
    }
    if (0x1715..0x1715 + 1).contains(&cp) {
        return true;
    }
    if (0x17C9..0x17D3 + 1).contains(&cp) {
        return true;
    }
    if (0x17DD..0x17DD + 1).contains(&cp) {
        return true;
    }
    if (0x1939..0x193B + 1).contains(&cp) {
        return true;
    }
    if (0x1A75..0x1A7C + 1).contains(&cp) {
        return true;
    }
    if (0x1A7F..0x1A7F + 1).contains(&cp) {
        return true;
    }
    if (0x1AB0..0x1ABD + 1).contains(&cp) {
        return true;
    }
    if (0x1ABE..0x1ABE + 1).contains(&cp) {
        return true;
    }
    if (0x1AC1..0x1ACB + 1).contains(&cp) {
        return true;
    }
    if (0x1B34..0x1B34 + 1).contains(&cp) {
        return true;
    }
    if (0x1B44..0x1B44 + 1).contains(&cp) {
        return true;
    }
    if (0x1B6B..0x1B73 + 1).contains(&cp) {
        return true;
    }
    if (0x1BAA..0x1BAA + 1).contains(&cp) {
        return true;
    }
    if (0x1BAB..0x1BAB + 1).contains(&cp) {
        return true;
    }
    if (0x1C36..0x1C37 + 1).contains(&cp) {
        return true;
    }
    if (0x1C78..0x1C7D + 1).contains(&cp) {
        return true;
    }
    if (0x1CD0..0x1CD2 + 1).contains(&cp) {
        return true;
    }
    if (0x1CD3..0x1CD3 + 1).contains(&cp) {
        return true;
    }
    if (0x1CD4..0x1CE0 + 1).contains(&cp) {
        return true;
    }
    if (0x1CE1..0x1CE1 + 1).contains(&cp) {
        return true;
    }
    if (0x1CE2..0x1CE8 + 1).contains(&cp) {
        return true;
    }
    if (0x1CED..0x1CED + 1).contains(&cp) {
        return true;
    }
    if (0x1CF4..0x1CF4 + 1).contains(&cp) {
        return true;
    }
    if (0x1CF7..0x1CF7 + 1).contains(&cp) {
        return true;
    }
    if (0x1CF8..0x1CF9 + 1).contains(&cp) {
        return true;
    }
    if (0x1D2C..0x1D6A + 1).contains(&cp) {
        return true;
    }
    if (0x1DC4..0x1DCF + 1).contains(&cp) {
        return true;
    }
    if (0x1DF5..0x1DFF + 1).contains(&cp) {
        return true;
    }
    if (0x1FBD..0x1FBD + 1).contains(&cp) {
        return true;
    }
    if (0x1FBF..0x1FC1 + 1).contains(&cp) {
        return true;
    }
    if (0x1FCD..0x1FCF + 1).contains(&cp) {
        return true;
    }
    if (0x1FDD..0x1FDF + 1).contains(&cp) {
        return true;
    }
    if (0x1FED..0x1FEF + 1).contains(&cp) {
        return true;
    }
    if (0x1FFD..0x1FFE + 1).contains(&cp) {
        return true;
    }
    if (0x2CEF..0x2CF1 + 1).contains(&cp) {
        return true;
    }
    if (0x2E2F..0x2E2F + 1).contains(&cp) {
        return true;
    }
    if (0x302A..0x302D + 1).contains(&cp) {
        return true;
    }
    if (0x302E..0x302F + 1).contains(&cp) {
        return true;
    }
    if (0x3099..0x309A + 1).contains(&cp) {
        return true;
    }
    if (0x309B..0x309C + 1).contains(&cp) {
        return true;
    }
    if (0x30FC..0x30FC + 1).contains(&cp) {
        return true;
    }
    if (0xA66F..0xA66F + 1).contains(&cp) {
        return true;
    }
    if (0xA67C..0xA67D + 1).contains(&cp) {
        return true;
    }
    if (0xA67F..0xA67F + 1).contains(&cp) {
        return true;
    }
    if (0xA69C..0xA69D + 1).contains(&cp) {
        return true;
    }
    if (0xA6F0..0xA6F1 + 1).contains(&cp) {
        return true;
    }
    if (0xA700..0xA716 + 1).contains(&cp) {
        return true;
    }
    if (0xA717..0xA71F + 1).contains(&cp) {
        return true;
    }
    if (0xA720..0xA721 + 1).contains(&cp) {
        return true;
    }
    if (0xA788..0xA788 + 1).contains(&cp) {
        return true;
    }
    if (0xA789..0xA78A + 1).contains(&cp) {
        return true;
    }
    if (0xA7F8..0xA7F9 + 1).contains(&cp) {
        return true;
    }
    if (0xA8C4..0xA8C4 + 1).contains(&cp) {
        return true;
    }
    if (0xA8E0..0xA8F1 + 1).contains(&cp) {
        return true;
    }
    if (0xA92B..0xA92D + 1).contains(&cp) {
        return true;
    }
    if (0xA92E..0xA92E + 1).contains(&cp) {
        return true;
    }
    if (0xA953..0xA953 + 1).contains(&cp) {
        return true;
    }
    if (0xA9B3..0xA9B3 + 1).contains(&cp) {
        return true;
    }
    if (0xA9C0..0xA9C0 + 1).contains(&cp) {
        return true;
    }
    if (0xA9E5..0xA9E5 + 1).contains(&cp) {
        return true;
    }
    if (0xAA7B..0xAA7B + 1).contains(&cp) {
        return true;
    }
    if (0xAA7C..0xAA7C + 1).contains(&cp) {
        return true;
    }
    if (0xAA7D..0xAA7D + 1).contains(&cp) {
        return true;
    }
    if (0xAABF..0xAABF + 1).contains(&cp) {
        return true;
    }
    if (0xAAC0..0xAAC0 + 1).contains(&cp) {
        return true;
    }
    if (0xAAC1..0xAAC1 + 1).contains(&cp) {
        return true;
    }
    if (0xAAC2..0xAAC2 + 1).contains(&cp) {
        return true;
    }
    if (0xAAF6..0xAAF6 + 1).contains(&cp) {
        return true;
    }
    if (0xAB5B..0xAB5B + 1).contains(&cp) {
        return true;
    }
    if (0xAB5C..0xAB5F + 1).contains(&cp) {
        return true;
    }
    if (0xAB69..0xAB69 + 1).contains(&cp) {
        return true;
    }
    if (0xAB6A..0xAB6B + 1).contains(&cp) {
        return true;
    }
    if (0xABEC..0xABEC + 1).contains(&cp) {
        return true;
    }
    if (0xABED..0xABED + 1).contains(&cp) {
        return true;
    }
    if (0xFB1E..0xFB1E + 1).contains(&cp) {
        return true;
    }
    if (0xFE20..0xFE2F + 1).contains(&cp) {
        return true;
    }
    if (0xFF3E..0xFF3E + 1).contains(&cp) {
        return true;
    }
    if (0xFF40..0xFF40 + 1).contains(&cp) {
        return true;
    }
    if (0xFF70..0xFF70 + 1).contains(&cp) {
        return true;
    }
    if (0xFF9E..0xFF9F + 1).contains(&cp) {
        return true;
    }
    if (0xFFE3..0xFFE3 + 1).contains(&cp) {
        return true;
    }
    if (0x102E0..0x102E0 + 1).contains(&cp) {
        return true;
    }
    if (0x10780..0x10785 + 1).contains(&cp) {
        return true;
    }
    if (0x10787..0x107B0 + 1).contains(&cp) {
        return true;
    }
    if (0x107B2..0x107BA + 1).contains(&cp) {
        return true;
    }
    if (0x10AE5..0x10AE6 + 1).contains(&cp) {
        return true;
    }
    if (0x10D22..0x10D23 + 1).contains(&cp) {
        return true;
    }
    if (0x10D24..0x10D27 + 1).contains(&cp) {
        return true;
    }
    if (0x10EFD..0x10EFF + 1).contains(&cp) {
        return true;
    }
    if (0x10F46..0x10F50 + 1).contains(&cp) {
        return true;
    }
    if (0x10F82..0x10F85 + 1).contains(&cp) {
        return true;
    }
    if (0x11046..0x11046 + 1).contains(&cp) {
        return true;
    }
    if (0x11070..0x11070 + 1).contains(&cp) {
        return true;
    }
    if (0x110B9..0x110BA + 1).contains(&cp) {
        return true;
    }
    if (0x11133..0x11134 + 1).contains(&cp) {
        return true;
    }
    if (0x11173..0x11173 + 1).contains(&cp) {
        return true;
    }
    if (0x111C0..0x111C0 + 1).contains(&cp) {
        return true;
    }
    if (0x111CA..0x111CC + 1).contains(&cp) {
        return true;
    }
    if (0x11235..0x11235 + 1).contains(&cp) {
        return true;
    }
    if (0x11236..0x11236 + 1).contains(&cp) {
        return true;
    }
    if (0x112E9..0x112EA + 1).contains(&cp) {
        return true;
    }
    if (0x1133C..0x1133C + 1).contains(&cp) {
        return true;
    }
    if (0x1134D..0x1134D + 1).contains(&cp) {
        return true;
    }
    if (0x11366..0x1136C + 1).contains(&cp) {
        return true;
    }
    if (0x11370..0x11374 + 1).contains(&cp) {
        return true;
    }
    if (0x11442..0x11442 + 1).contains(&cp) {
        return true;
    }
    if (0x11446..0x11446 + 1).contains(&cp) {
        return true;
    }
    if (0x114C2..0x114C3 + 1).contains(&cp) {
        return true;
    }
    if (0x115BF..0x115C0 + 1).contains(&cp) {
        return true;
    }
    if (0x1163F..0x1163F + 1).contains(&cp) {
        return true;
    }
    if (0x116B6..0x116B6 + 1).contains(&cp) {
        return true;
    }
    if (0x116B7..0x116B7 + 1).contains(&cp) {
        return true;
    }
    if (0x1172B..0x1172B + 1).contains(&cp) {
        return true;
    }
    if (0x11839..0x1183A + 1).contains(&cp) {
        return true;
    }
    if (0x1193D..0x1193D + 1).contains(&cp) {
        return true;
    }
    if (0x1193E..0x1193E + 1).contains(&cp) {
        return true;
    }
    if (0x11943..0x11943 + 1).contains(&cp) {
        return true;
    }
    if (0x119E0..0x119E0 + 1).contains(&cp) {
        return true;
    }
    if (0x11A34..0x11A34 + 1).contains(&cp) {
        return true;
    }
    if (0x11A47..0x11A47 + 1).contains(&cp) {
        return true;
    }
    if (0x11A99..0x11A99 + 1).contains(&cp) {
        return true;
    }
    if (0x11C3F..0x11C3F + 1).contains(&cp) {
        return true;
    }
    if (0x11D42..0x11D42 + 1).contains(&cp) {
        return true;
    }
    if (0x11D44..0x11D45 + 1).contains(&cp) {
        return true;
    }
    if (0x11D97..0x11D97 + 1).contains(&cp) {
        return true;
    }
    if (0x13447..0x13455 + 1).contains(&cp) {
        return true;
    }
    if (0x16AF0..0x16AF4 + 1).contains(&cp) {
        return true;
    }
    if (0x16B30..0x16B36 + 1).contains(&cp) {
        return true;
    }
    if (0x16F8F..0x16F92 + 1).contains(&cp) {
        return true;
    }
    if (0x16F93..0x16F9F + 1).contains(&cp) {
        return true;
    }
    if (0x16FF0..0x16FF1 + 1).contains(&cp) {
        return true;
    }
    if (0x1AFF0..0x1AFF3 + 1).contains(&cp) {
        return true;
    }
    if (0x1AFF5..0x1AFFB + 1).contains(&cp) {
        return true;
    }
    if (0x1AFFD..0x1AFFE + 1).contains(&cp) {
        return true;
    }
    if (0x1CF00..0x1CF2D + 1).contains(&cp) {
        return true;
    }
    if (0x1CF30..0x1CF46 + 1).contains(&cp) {
        return true;
    }
    if (0x1D167..0x1D169 + 1).contains(&cp) {
        return true;
    }
    if (0x1D16D..0x1D172 + 1).contains(&cp) {
        return true;
    }
    if (0x1D17B..0x1D182 + 1).contains(&cp) {
        return true;
    }
    if (0x1D185..0x1D18B + 1).contains(&cp) {
        return true;
    }
    if (0x1D1AA..0x1D1AD + 1).contains(&cp) {
        return true;
    }
    if (0x1E030..0x1E06D + 1).contains(&cp) {
        return true;
    }
    if (0x1E130..0x1E136 + 1).contains(&cp) {
        return true;
    }
    if (0x1E2AE..0x1E2AE + 1).contains(&cp) {
        return true;
    }
    if (0x1E2EC..0x1E2EF + 1).contains(&cp) {
        return true;
    }
    if (0x1E8D0..0x1E8D6 + 1).contains(&cp) {
        return true;
    }
    if (0x1E944..0x1E946 + 1).contains(&cp) {
        return true;
    }
    if (0x1E948..0x1E94A + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn ext(cp: u32) -> bool {
    if (0x00B7..0x00B7 + 1).contains(&cp) {
        return true;
    }
    if (0x02D0..0x02D1 + 1).contains(&cp) {
        return true;
    }
    if (0x0640..0x0640 + 1).contains(&cp) {
        return true;
    }
    if (0x07FA..0x07FA + 1).contains(&cp) {
        return true;
    }
    if (0x0B55..0x0B55 + 1).contains(&cp) {
        return true;
    }
    if (0x0E46..0x0E46 + 1).contains(&cp) {
        return true;
    }
    if (0x0EC6..0x0EC6 + 1).contains(&cp) {
        return true;
    }
    if (0x180A..0x180A + 1).contains(&cp) {
        return true;
    }
    if (0x1843..0x1843 + 1).contains(&cp) {
        return true;
    }
    if (0x1AA7..0x1AA7 + 1).contains(&cp) {
        return true;
    }
    if (0x1C36..0x1C36 + 1).contains(&cp) {
        return true;
    }
    if (0x1C7B..0x1C7B + 1).contains(&cp) {
        return true;
    }
    if (0x3005..0x3005 + 1).contains(&cp) {
        return true;
    }
    if (0x3031..0x3035 + 1).contains(&cp) {
        return true;
    }
    if (0x309D..0x309E + 1).contains(&cp) {
        return true;
    }
    if (0x30FC..0x30FE + 1).contains(&cp) {
        return true;
    }
    if (0xA015..0xA015 + 1).contains(&cp) {
        return true;
    }
    if (0xA60C..0xA60C + 1).contains(&cp) {
        return true;
    }
    if (0xA9CF..0xA9CF + 1).contains(&cp) {
        return true;
    }
    if (0xA9E6..0xA9E6 + 1).contains(&cp) {
        return true;
    }
    if (0xAA70..0xAA70 + 1).contains(&cp) {
        return true;
    }
    if (0xAADD..0xAADD + 1).contains(&cp) {
        return true;
    }
    if (0xAAF3..0xAAF4 + 1).contains(&cp) {
        return true;
    }
    if (0xFF70..0xFF70 + 1).contains(&cp) {
        return true;
    }
    if (0x10781..0x10782 + 1).contains(&cp) {
        return true;
    }
    if (0x1135D..0x1135D + 1).contains(&cp) {
        return true;
    }
    if (0x115C6..0x115C8 + 1).contains(&cp) {
        return true;
    }
    if (0x11A98..0x11A98 + 1).contains(&cp) {
        return true;
    }
    if (0x16B42..0x16B43 + 1).contains(&cp) {
        return true;
    }
    if (0x16FE0..0x16FE1 + 1).contains(&cp) {
        return true;
    }
    if (0x16FE3..0x16FE3 + 1).contains(&cp) {
        return true;
    }
    if (0x1E13C..0x1E13D + 1).contains(&cp) {
        return true;
    }
    if (0x1E944..0x1E946 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn olower(cp: u32) -> bool {
    if (0x00AA..0x00AA + 1).contains(&cp) {
        return true;
    }
    if (0x00BA..0x00BA + 1).contains(&cp) {
        return true;
    }
    if (0x02B0..0x02B8 + 1).contains(&cp) {
        return true;
    }
    if (0x02C0..0x02C1 + 1).contains(&cp) {
        return true;
    }
    if (0x02E0..0x02E4 + 1).contains(&cp) {
        return true;
    }
    if (0x0345..0x0345 + 1).contains(&cp) {
        return true;
    }
    if (0x037A..0x037A + 1).contains(&cp) {
        return true;
    }
    if (0x10FC..0x10FC + 1).contains(&cp) {
        return true;
    }
    if (0x1D2C..0x1D6A + 1).contains(&cp) {
        return true;
    }
    if (0x1D78..0x1D78 + 1).contains(&cp) {
        return true;
    }
    if (0x1D9B..0x1DBF + 1).contains(&cp) {
        return true;
    }
    if (0x2071..0x2071 + 1).contains(&cp) {
        return true;
    }
    if (0x207F..0x207F + 1).contains(&cp) {
        return true;
    }
    if (0x2090..0x209C + 1).contains(&cp) {
        return true;
    }
    if (0x2170..0x217F + 1).contains(&cp) {
        return true;
    }
    if (0x24D0..0x24E9 + 1).contains(&cp) {
        return true;
    }
    if (0x2C7C..0x2C7D + 1).contains(&cp) {
        return true;
    }
    if (0xA69C..0xA69D + 1).contains(&cp) {
        return true;
    }
    if (0xA770..0xA770 + 1).contains(&cp) {
        return true;
    }
    if (0xA7F2..0xA7F4 + 1).contains(&cp) {
        return true;
    }
    if (0xA7F8..0xA7F9 + 1).contains(&cp) {
        return true;
    }
    if (0xAB5C..0xAB5F + 1).contains(&cp) {
        return true;
    }
    if (0xAB69..0xAB69 + 1).contains(&cp) {
        return true;
    }
    if (0x10780..0x10780 + 1).contains(&cp) {
        return true;
    }
    if (0x10783..0x10785 + 1).contains(&cp) {
        return true;
    }
    if (0x10787..0x107B0 + 1).contains(&cp) {
        return true;
    }
    if (0x107B2..0x107BA + 1).contains(&cp) {
        return true;
    }
    if (0x1E030..0x1E06D + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn oupper(cp: u32) -> bool {
    if (0x2160..0x216F + 1).contains(&cp) {
        return true;
    }
    if (0x24B6..0x24CF + 1).contains(&cp) {
        return true;
    }
    if (0x1F130..0x1F149 + 1).contains(&cp) {
        return true;
    }
    if (0x1F150..0x1F169 + 1).contains(&cp) {
        return true;
    }
    if (0x1F170..0x1F189 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn nchar(cp: u32) -> bool {
    if (0xFDD0..0xFDEF + 1).contains(&cp) {
        return true;
    }
    if (0xFFFE..0xFFFF + 1).contains(&cp) {
        return true;
    }
    if (0x1FFFE..0x1FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x2FFFE..0x2FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x3FFFE..0x3FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x4FFFE..0x4FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x5FFFE..0x5FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x6FFFE..0x6FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x7FFFE..0x7FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x8FFFE..0x8FFFF + 1).contains(&cp) {
        return true;
    }
    if (0x9FFFE..0x9FFFF + 1).contains(&cp) {
        return true;
    }
    if (0xAFFFE..0xAFFFF + 1).contains(&cp) {
        return true;
    }
    if (0xBFFFE..0xBFFFF + 1).contains(&cp) {
        return true;
    }
    if (0xCFFFE..0xCFFFF + 1).contains(&cp) {
        return true;
    }
    if (0xDFFFE..0xDFFFF + 1).contains(&cp) {
        return true;
    }
    if (0xEFFFE..0xEFFFF + 1).contains(&cp) {
        return true;
    }
    if (0xFFFFE..0xFFFFF + 1).contains(&cp) {
        return true;
    }
    if (0x10FFFE..0x10FFFF + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn ogr_ext(cp: u32) -> bool {
    if (0x09BE..0x09BE + 1).contains(&cp) {
        return true;
    }
    if (0x09D7..0x09D7 + 1).contains(&cp) {
        return true;
    }
    if (0x0B3E..0x0B3E + 1).contains(&cp) {
        return true;
    }
    if (0x0B57..0x0B57 + 1).contains(&cp) {
        return true;
    }
    if (0x0BBE..0x0BBE + 1).contains(&cp) {
        return true;
    }
    if (0x0BD7..0x0BD7 + 1).contains(&cp) {
        return true;
    }
    if (0x0CC2..0x0CC2 + 1).contains(&cp) {
        return true;
    }
    if (0x0CD5..0x0CD6 + 1).contains(&cp) {
        return true;
    }
    if (0x0D3E..0x0D3E + 1).contains(&cp) {
        return true;
    }
    if (0x0D57..0x0D57 + 1).contains(&cp) {
        return true;
    }
    if (0x0DCF..0x0DCF + 1).contains(&cp) {
        return true;
    }
    if (0x0DDF..0x0DDF + 1).contains(&cp) {
        return true;
    }
    if (0x1B35..0x1B35 + 1).contains(&cp) {
        return true;
    }
    if (0x200C..0x200C + 1).contains(&cp) {
        return true;
    }
    if (0x302E..0x302F + 1).contains(&cp) {
        return true;
    }
    if (0xFF9E..0xFF9F + 1).contains(&cp) {
        return true;
    }
    if (0x1133E..0x1133E + 1).contains(&cp) {
        return true;
    }
    if (0x11357..0x11357 + 1).contains(&cp) {
        return true;
    }
    if (0x114B0..0x114B0 + 1).contains(&cp) {
        return true;
    }
    if (0x114BD..0x114BD + 1).contains(&cp) {
        return true;
    }
    if (0x115AF..0x115AF + 1).contains(&cp) {
        return true;
    }
    if (0x11930..0x11930 + 1).contains(&cp) {
        return true;
    }
    if (0x1D165..0x1D165 + 1).contains(&cp) {
        return true;
    }
    if (0x1D16E..0x1D172 + 1).contains(&cp) {
        return true;
    }
    if (0xE0020..0xE007F + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn idsb(cp: u32) -> bool {
    if (0x2FF0..0x2FF1 + 1).contains(&cp) {
        return true;
    }
    if (0x2FF4..0x2FFB + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn idst(cp: u32) -> bool {
    if (0x2FF2..0x2FF3 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn radical(cp: u32) -> bool {
    if (0x2E80..0x2E99 + 1).contains(&cp) {
        return true;
    }
    if (0x2E9B..0x2EF3 + 1).contains(&cp) {
        return true;
    }
    if (0x2F00..0x2FD5 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn uideo(cp: u32) -> bool {
    if (0x3400..0x4DBF + 1).contains(&cp) {
        return true;
    }
    if (0x4E00..0x9FFF + 1).contains(&cp) {
        return true;
    }
    if (0xFA0E..0xFA0F + 1).contains(&cp) {
        return true;
    }
    if (0xFA11..0xFA11 + 1).contains(&cp) {
        return true;
    }
    if (0xFA13..0xFA14 + 1).contains(&cp) {
        return true;
    }
    if (0xFA1F..0xFA1F + 1).contains(&cp) {
        return true;
    }
    if (0xFA21..0xFA21 + 1).contains(&cp) {
        return true;
    }
    if (0xFA23..0xFA24 + 1).contains(&cp) {
        return true;
    }
    if (0xFA27..0xFA29 + 1).contains(&cp) {
        return true;
    }
    if (0x20000..0x2A6DF + 1).contains(&cp) {
        return true;
    }
    if (0x2A700..0x2B739 + 1).contains(&cp) {
        return true;
    }
    if (0x2B740..0x2B81D + 1).contains(&cp) {
        return true;
    }
    if (0x2B820..0x2CEA1 + 1).contains(&cp) {
        return true;
    }
    if (0x2CEB0..0x2EBE0 + 1).contains(&cp) {
        return true;
    }
    if (0x30000..0x3134A + 1).contains(&cp) {
        return true;
    }
    if (0x31350..0x323AF + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn odi(cp: u32) -> bool {
    if (0x034F..0x034F + 1).contains(&cp) {
        return true;
    }
    if (0x115F..0x1160 + 1).contains(&cp) {
        return true;
    }
    if (0x17B4..0x17B5 + 1).contains(&cp) {
        return true;
    }
    if (0x2065..0x2065 + 1).contains(&cp) {
        return true;
    }
    if (0x3164..0x3164 + 1).contains(&cp) {
        return true;
    }
    if (0xFFA0..0xFFA0 + 1).contains(&cp) {
        return true;
    }
    if (0xFFF0..0xFFF8 + 1).contains(&cp) {
        return true;
    }
    if (0xE0000..0xE0000 + 1).contains(&cp) {
        return true;
    }
    if (0xE0002..0xE001F + 1).contains(&cp) {
        return true;
    }
    if (0xE0080..0xE00FF + 1).contains(&cp) {
        return true;
    }
    if (0xE01F0..0xE0FFF + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn dep(cp: u32) -> bool {
    if (0x0149..0x0149 + 1).contains(&cp) {
        return true;
    }
    if (0x0673..0x0673 + 1).contains(&cp) {
        return true;
    }
    if (0x0F77..0x0F77 + 1).contains(&cp) {
        return true;
    }
    if (0x0F79..0x0F79 + 1).contains(&cp) {
        return true;
    }
    if (0x17A3..0x17A4 + 1).contains(&cp) {
        return true;
    }
    if (0x206A..0x206F + 1).contains(&cp) {
        return true;
    }
    if (0x2329..0x2329 + 1).contains(&cp) {
        return true;
    }
    if (0x232A..0x232A + 1).contains(&cp) {
        return true;
    }
    if (0xE0001..0xE0001 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn sd(cp: u32) -> bool {
    if (0x0069..0x006A + 1).contains(&cp) {
        return true;
    }
    if (0x012F..0x012F + 1).contains(&cp) {
        return true;
    }
    if (0x0249..0x0249 + 1).contains(&cp) {
        return true;
    }
    if (0x0268..0x0268 + 1).contains(&cp) {
        return true;
    }
    if (0x029D..0x029D + 1).contains(&cp) {
        return true;
    }
    if (0x02B2..0x02B2 + 1).contains(&cp) {
        return true;
    }
    if (0x03F3..0x03F3 + 1).contains(&cp) {
        return true;
    }
    if (0x0456..0x0456 + 1).contains(&cp) {
        return true;
    }
    if (0x0458..0x0458 + 1).contains(&cp) {
        return true;
    }
    if (0x1D62..0x1D62 + 1).contains(&cp) {
        return true;
    }
    if (0x1D96..0x1D96 + 1).contains(&cp) {
        return true;
    }
    if (0x1DA4..0x1DA4 + 1).contains(&cp) {
        return true;
    }
    if (0x1DA8..0x1DA8 + 1).contains(&cp) {
        return true;
    }
    if (0x1E2D..0x1E2D + 1).contains(&cp) {
        return true;
    }
    if (0x1ECB..0x1ECB + 1).contains(&cp) {
        return true;
    }
    if (0x2071..0x2071 + 1).contains(&cp) {
        return true;
    }
    if (0x2148..0x2149 + 1).contains(&cp) {
        return true;
    }
    if (0x2C7C..0x2C7C + 1).contains(&cp) {
        return true;
    }
    if (0x1D422..0x1D423 + 1).contains(&cp) {
        return true;
    }
    if (0x1D456..0x1D457 + 1).contains(&cp) {
        return true;
    }
    if (0x1D48A..0x1D48B + 1).contains(&cp) {
        return true;
    }
    if (0x1D4BE..0x1D4BF + 1).contains(&cp) {
        return true;
    }
    if (0x1D4F2..0x1D4F3 + 1).contains(&cp) {
        return true;
    }
    if (0x1D526..0x1D527 + 1).contains(&cp) {
        return true;
    }
    if (0x1D55A..0x1D55B + 1).contains(&cp) {
        return true;
    }
    if (0x1D58E..0x1D58F + 1).contains(&cp) {
        return true;
    }
    if (0x1D5C2..0x1D5C3 + 1).contains(&cp) {
        return true;
    }
    if (0x1D5F6..0x1D5F7 + 1).contains(&cp) {
        return true;
    }
    if (0x1D62A..0x1D62B + 1).contains(&cp) {
        return true;
    }
    if (0x1D65E..0x1D65F + 1).contains(&cp) {
        return true;
    }
    if (0x1D692..0x1D693 + 1).contains(&cp) {
        return true;
    }
    if (0x1DF1A..0x1DF1A + 1).contains(&cp) {
        return true;
    }
    if (0x1E04C..0x1E04D + 1).contains(&cp) {
        return true;
    }
    if (0x1E068..0x1E068 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn loe(cp: u32) -> bool {
    if (0x0E40..0x0E44 + 1).contains(&cp) {
        return true;
    }
    if (0x0EC0..0x0EC4 + 1).contains(&cp) {
        return true;
    }
    if (0x19B5..0x19B7 + 1).contains(&cp) {
        return true;
    }
    if (0x19BA..0x19BA + 1).contains(&cp) {
        return true;
    }
    if (0xAAB5..0xAAB6 + 1).contains(&cp) {
        return true;
    }
    if (0xAAB9..0xAAB9 + 1).contains(&cp) {
        return true;
    }
    if (0xAABB..0xAABC + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn oids(cp: u32) -> bool {
    if (0x1885..0x1886 + 1).contains(&cp) {
        return true;
    }
    if (0x2118..0x2118 + 1).contains(&cp) {
        return true;
    }
    if (0x212E..0x212E + 1).contains(&cp) {
        return true;
    }
    if (0x309B..0x309C + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn oidc(cp: u32) -> bool {
    if (0x00B7..0x00B7 + 1).contains(&cp) {
        return true;
    }
    if (0x0387..0x0387 + 1).contains(&cp) {
        return true;
    }
    if (0x1369..0x1371 + 1).contains(&cp) {
        return true;
    }
    if (0x19DA..0x19DA + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn sterm(cp: u32) -> bool {
    if (0x0021..0x0021 + 1).contains(&cp) {
        return true;
    }
    if (0x002E..0x002E + 1).contains(&cp) {
        return true;
    }
    if (0x003F..0x003F + 1).contains(&cp) {
        return true;
    }
    if (0x0589..0x0589 + 1).contains(&cp) {
        return true;
    }
    if (0x061D..0x061F + 1).contains(&cp) {
        return true;
    }
    if (0x06D4..0x06D4 + 1).contains(&cp) {
        return true;
    }
    if (0x0700..0x0702 + 1).contains(&cp) {
        return true;
    }
    if (0x07F9..0x07F9 + 1).contains(&cp) {
        return true;
    }
    if (0x0837..0x0837 + 1).contains(&cp) {
        return true;
    }
    if (0x0839..0x0839 + 1).contains(&cp) {
        return true;
    }
    if (0x083D..0x083E + 1).contains(&cp) {
        return true;
    }
    if (0x0964..0x0965 + 1).contains(&cp) {
        return true;
    }
    if (0x104A..0x104B + 1).contains(&cp) {
        return true;
    }
    if (0x1362..0x1362 + 1).contains(&cp) {
        return true;
    }
    if (0x1367..0x1368 + 1).contains(&cp) {
        return true;
    }
    if (0x166E..0x166E + 1).contains(&cp) {
        return true;
    }
    if (0x1735..0x1736 + 1).contains(&cp) {
        return true;
    }
    if (0x1803..0x1803 + 1).contains(&cp) {
        return true;
    }
    if (0x1809..0x1809 + 1).contains(&cp) {
        return true;
    }
    if (0x1944..0x1945 + 1).contains(&cp) {
        return true;
    }
    if (0x1AA8..0x1AAB + 1).contains(&cp) {
        return true;
    }
    if (0x1B5A..0x1B5B + 1).contains(&cp) {
        return true;
    }
    if (0x1B5E..0x1B5F + 1).contains(&cp) {
        return true;
    }
    if (0x1B7D..0x1B7E + 1).contains(&cp) {
        return true;
    }
    if (0x1C3B..0x1C3C + 1).contains(&cp) {
        return true;
    }
    if (0x1C7E..0x1C7F + 1).contains(&cp) {
        return true;
    }
    if (0x203C..0x203D + 1).contains(&cp) {
        return true;
    }
    if (0x2047..0x2049 + 1).contains(&cp) {
        return true;
    }
    if (0x2E2E..0x2E2E + 1).contains(&cp) {
        return true;
    }
    if (0x2E3C..0x2E3C + 1).contains(&cp) {
        return true;
    }
    if (0x2E53..0x2E54 + 1).contains(&cp) {
        return true;
    }
    if (0x3002..0x3002 + 1).contains(&cp) {
        return true;
    }
    if (0xA4FF..0xA4FF + 1).contains(&cp) {
        return true;
    }
    if (0xA60E..0xA60F + 1).contains(&cp) {
        return true;
    }
    if (0xA6F3..0xA6F3 + 1).contains(&cp) {
        return true;
    }
    if (0xA6F7..0xA6F7 + 1).contains(&cp) {
        return true;
    }
    if (0xA876..0xA877 + 1).contains(&cp) {
        return true;
    }
    if (0xA8CE..0xA8CF + 1).contains(&cp) {
        return true;
    }
    if (0xA92F..0xA92F + 1).contains(&cp) {
        return true;
    }
    if (0xA9C8..0xA9C9 + 1).contains(&cp) {
        return true;
    }
    if (0xAA5D..0xAA5F + 1).contains(&cp) {
        return true;
    }
    if (0xAAF0..0xAAF1 + 1).contains(&cp) {
        return true;
    }
    if (0xABEB..0xABEB + 1).contains(&cp) {
        return true;
    }
    if (0xFE52..0xFE52 + 1).contains(&cp) {
        return true;
    }
    if (0xFE56..0xFE57 + 1).contains(&cp) {
        return true;
    }
    if (0xFF01..0xFF01 + 1).contains(&cp) {
        return true;
    }
    if (0xFF0E..0xFF0E + 1).contains(&cp) {
        return true;
    }
    if (0xFF1F..0xFF1F + 1).contains(&cp) {
        return true;
    }
    if (0xFF61..0xFF61 + 1).contains(&cp) {
        return true;
    }
    if (0x10A56..0x10A57 + 1).contains(&cp) {
        return true;
    }
    if (0x10F55..0x10F59 + 1).contains(&cp) {
        return true;
    }
    if (0x10F86..0x10F89 + 1).contains(&cp) {
        return true;
    }
    if (0x11047..0x11048 + 1).contains(&cp) {
        return true;
    }
    if (0x110BE..0x110C1 + 1).contains(&cp) {
        return true;
    }
    if (0x11141..0x11143 + 1).contains(&cp) {
        return true;
    }
    if (0x111C5..0x111C6 + 1).contains(&cp) {
        return true;
    }
    if (0x111CD..0x111CD + 1).contains(&cp) {
        return true;
    }
    if (0x111DE..0x111DF + 1).contains(&cp) {
        return true;
    }
    if (0x11238..0x11239 + 1).contains(&cp) {
        return true;
    }
    if (0x1123B..0x1123C + 1).contains(&cp) {
        return true;
    }
    if (0x112A9..0x112A9 + 1).contains(&cp) {
        return true;
    }
    if (0x1144B..0x1144C + 1).contains(&cp) {
        return true;
    }
    if (0x115C2..0x115C3 + 1).contains(&cp) {
        return true;
    }
    if (0x115C9..0x115D7 + 1).contains(&cp) {
        return true;
    }
    if (0x11641..0x11642 + 1).contains(&cp) {
        return true;
    }
    if (0x1173C..0x1173E + 1).contains(&cp) {
        return true;
    }
    if (0x11944..0x11944 + 1).contains(&cp) {
        return true;
    }
    if (0x11946..0x11946 + 1).contains(&cp) {
        return true;
    }
    if (0x11A42..0x11A43 + 1).contains(&cp) {
        return true;
    }
    if (0x11A9B..0x11A9C + 1).contains(&cp) {
        return true;
    }
    if (0x11C41..0x11C42 + 1).contains(&cp) {
        return true;
    }
    if (0x11EF7..0x11EF8 + 1).contains(&cp) {
        return true;
    }
    if (0x11F43..0x11F44 + 1).contains(&cp) {
        return true;
    }
    if (0x16A6E..0x16A6F + 1).contains(&cp) {
        return true;
    }
    if (0x16AF5..0x16AF5 + 1).contains(&cp) {
        return true;
    }
    if (0x16B37..0x16B38 + 1).contains(&cp) {
        return true;
    }
    if (0x16B44..0x16B44 + 1).contains(&cp) {
        return true;
    }
    if (0x16E98..0x16E98 + 1).contains(&cp) {
        return true;
    }
    if (0x1BC9F..0x1BC9F + 1).contains(&cp) {
        return true;
    }
    if (0x1DA88..0x1DA88 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn vs(cp: u32) -> bool {
    if (0x180B..0x180D + 1).contains(&cp) {
        return true;
    }
    if (0x180F..0x180F + 1).contains(&cp) {
        return true;
    }
    if (0xFE00..0xFE0F + 1).contains(&cp) {
        return true;
    }
    if (0xE0100..0xE01EF + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn pat_ws(cp: u32) -> bool {
    if (0x0009..0x000D + 1).contains(&cp) {
        return true;
    }
    if (0x0020..0x0020 + 1).contains(&cp) {
        return true;
    }
    if (0x0085..0x0085 + 1).contains(&cp) {
        return true;
    }
    if (0x200E..0x200F + 1).contains(&cp) {
        return true;
    }
    if (0x2028..0x2028 + 1).contains(&cp) {
        return true;
    }
    if (0x2029..0x2029 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn pat_syn(cp: u32) -> bool {
    if (0x0021..0x0023 + 1).contains(&cp) {
        return true;
    }
    if (0x0024..0x0024 + 1).contains(&cp) {
        return true;
    }
    if (0x0025..0x0027 + 1).contains(&cp) {
        return true;
    }
    if (0x0028..0x0028 + 1).contains(&cp) {
        return true;
    }
    if (0x0029..0x0029 + 1).contains(&cp) {
        return true;
    }
    if (0x002A..0x002A + 1).contains(&cp) {
        return true;
    }
    if (0x002B..0x002B + 1).contains(&cp) {
        return true;
    }
    if (0x002C..0x002C + 1).contains(&cp) {
        return true;
    }
    if (0x002D..0x002D + 1).contains(&cp) {
        return true;
    }
    if (0x002E..0x002F + 1).contains(&cp) {
        return true;
    }
    if (0x003A..0x003B + 1).contains(&cp) {
        return true;
    }
    if (0x003C..0x003E + 1).contains(&cp) {
        return true;
    }
    if (0x003F..0x0040 + 1).contains(&cp) {
        return true;
    }
    if (0x005B..0x005B + 1).contains(&cp) {
        return true;
    }
    if (0x005C..0x005C + 1).contains(&cp) {
        return true;
    }
    if (0x005D..0x005D + 1).contains(&cp) {
        return true;
    }
    if (0x005E..0x005E + 1).contains(&cp) {
        return true;
    }
    if (0x0060..0x0060 + 1).contains(&cp) {
        return true;
    }
    if (0x007B..0x007B + 1).contains(&cp) {
        return true;
    }
    if (0x007C..0x007C + 1).contains(&cp) {
        return true;
    }
    if (0x007D..0x007D + 1).contains(&cp) {
        return true;
    }
    if (0x007E..0x007E + 1).contains(&cp) {
        return true;
    }
    if (0x00A1..0x00A1 + 1).contains(&cp) {
        return true;
    }
    if (0x00A2..0x00A5 + 1).contains(&cp) {
        return true;
    }
    if (0x00A6..0x00A6 + 1).contains(&cp) {
        return true;
    }
    if (0x00A7..0x00A7 + 1).contains(&cp) {
        return true;
    }
    if (0x00A9..0x00A9 + 1).contains(&cp) {
        return true;
    }
    if (0x00AB..0x00AB + 1).contains(&cp) {
        return true;
    }
    if (0x00AC..0x00AC + 1).contains(&cp) {
        return true;
    }
    if (0x00AE..0x00AE + 1).contains(&cp) {
        return true;
    }
    if (0x00B0..0x00B0 + 1).contains(&cp) {
        return true;
    }
    if (0x00B1..0x00B1 + 1).contains(&cp) {
        return true;
    }
    if (0x00B6..0x00B6 + 1).contains(&cp) {
        return true;
    }
    if (0x00BB..0x00BB + 1).contains(&cp) {
        return true;
    }
    if (0x00BF..0x00BF + 1).contains(&cp) {
        return true;
    }
    if (0x00D7..0x00D7 + 1).contains(&cp) {
        return true;
    }
    if (0x00F7..0x00F7 + 1).contains(&cp) {
        return true;
    }
    if (0x2010..0x2015 + 1).contains(&cp) {
        return true;
    }
    if (0x2016..0x2017 + 1).contains(&cp) {
        return true;
    }
    if (0x2018..0x2018 + 1).contains(&cp) {
        return true;
    }
    if (0x2019..0x2019 + 1).contains(&cp) {
        return true;
    }
    if (0x201A..0x201A + 1).contains(&cp) {
        return true;
    }
    if (0x201B..0x201C + 1).contains(&cp) {
        return true;
    }
    if (0x201D..0x201D + 1).contains(&cp) {
        return true;
    }
    if (0x201E..0x201E + 1).contains(&cp) {
        return true;
    }
    if (0x201F..0x201F + 1).contains(&cp) {
        return true;
    }
    if (0x2020..0x2027 + 1).contains(&cp) {
        return true;
    }
    if (0x2030..0x2038 + 1).contains(&cp) {
        return true;
    }
    if (0x2039..0x2039 + 1).contains(&cp) {
        return true;
    }
    if (0x203A..0x203A + 1).contains(&cp) {
        return true;
    }
    if (0x203B..0x203E + 1).contains(&cp) {
        return true;
    }
    if (0x2041..0x2043 + 1).contains(&cp) {
        return true;
    }
    if (0x2044..0x2044 + 1).contains(&cp) {
        return true;
    }
    if (0x2045..0x2045 + 1).contains(&cp) {
        return true;
    }
    if (0x2046..0x2046 + 1).contains(&cp) {
        return true;
    }
    if (0x2047..0x2051 + 1).contains(&cp) {
        return true;
    }
    if (0x2052..0x2052 + 1).contains(&cp) {
        return true;
    }
    if (0x2053..0x2053 + 1).contains(&cp) {
        return true;
    }
    if (0x2055..0x205E + 1).contains(&cp) {
        return true;
    }
    if (0x2190..0x2194 + 1).contains(&cp) {
        return true;
    }
    if (0x2195..0x2199 + 1).contains(&cp) {
        return true;
    }
    if (0x219A..0x219B + 1).contains(&cp) {
        return true;
    }
    if (0x219C..0x219F + 1).contains(&cp) {
        return true;
    }
    if (0x21A0..0x21A0 + 1).contains(&cp) {
        return true;
    }
    if (0x21A1..0x21A2 + 1).contains(&cp) {
        return true;
    }
    if (0x21A3..0x21A3 + 1).contains(&cp) {
        return true;
    }
    if (0x21A4..0x21A5 + 1).contains(&cp) {
        return true;
    }
    if (0x21A6..0x21A6 + 1).contains(&cp) {
        return true;
    }
    if (0x21A7..0x21AD + 1).contains(&cp) {
        return true;
    }
    if (0x21AE..0x21AE + 1).contains(&cp) {
        return true;
    }
    if (0x21AF..0x21CD + 1).contains(&cp) {
        return true;
    }
    if (0x21CE..0x21CF + 1).contains(&cp) {
        return true;
    }
    if (0x21D0..0x21D1 + 1).contains(&cp) {
        return true;
    }
    if (0x21D2..0x21D2 + 1).contains(&cp) {
        return true;
    }
    if (0x21D3..0x21D3 + 1).contains(&cp) {
        return true;
    }
    if (0x21D4..0x21D4 + 1).contains(&cp) {
        return true;
    }
    if (0x21D5..0x21F3 + 1).contains(&cp) {
        return true;
    }
    if (0x21F4..0x22FF + 1).contains(&cp) {
        return true;
    }
    if (0x2300..0x2307 + 1).contains(&cp) {
        return true;
    }
    if (0x2308..0x2308 + 1).contains(&cp) {
        return true;
    }
    if (0x2309..0x2309 + 1).contains(&cp) {
        return true;
    }
    if (0x230A..0x230A + 1).contains(&cp) {
        return true;
    }
    if (0x230B..0x230B + 1).contains(&cp) {
        return true;
    }
    if (0x230C..0x231F + 1).contains(&cp) {
        return true;
    }
    if (0x2320..0x2321 + 1).contains(&cp) {
        return true;
    }
    if (0x2322..0x2328 + 1).contains(&cp) {
        return true;
    }
    if (0x2329..0x2329 + 1).contains(&cp) {
        return true;
    }
    if (0x232A..0x232A + 1).contains(&cp) {
        return true;
    }
    if (0x232B..0x237B + 1).contains(&cp) {
        return true;
    }
    if (0x237C..0x237C + 1).contains(&cp) {
        return true;
    }
    if (0x237D..0x239A + 1).contains(&cp) {
        return true;
    }
    if (0x239B..0x23B3 + 1).contains(&cp) {
        return true;
    }
    if (0x23B4..0x23DB + 1).contains(&cp) {
        return true;
    }
    if (0x23DC..0x23E1 + 1).contains(&cp) {
        return true;
    }
    if (0x23E2..0x2426 + 1).contains(&cp) {
        return true;
    }
    if (0x2427..0x243F + 1).contains(&cp) {
        return true;
    }
    if (0x2440..0x244A + 1).contains(&cp) {
        return true;
    }
    if (0x244B..0x245F + 1).contains(&cp) {
        return true;
    }
    if (0x2500..0x25B6 + 1).contains(&cp) {
        return true;
    }
    if (0x25B7..0x25B7 + 1).contains(&cp) {
        return true;
    }
    if (0x25B8..0x25C0 + 1).contains(&cp) {
        return true;
    }
    if (0x25C1..0x25C1 + 1).contains(&cp) {
        return true;
    }
    if (0x25C2..0x25F7 + 1).contains(&cp) {
        return true;
    }
    if (0x25F8..0x25FF + 1).contains(&cp) {
        return true;
    }
    if (0x2600..0x266E + 1).contains(&cp) {
        return true;
    }
    if (0x266F..0x266F + 1).contains(&cp) {
        return true;
    }
    if (0x2670..0x2767 + 1).contains(&cp) {
        return true;
    }
    if (0x2768..0x2768 + 1).contains(&cp) {
        return true;
    }
    if (0x2769..0x2769 + 1).contains(&cp) {
        return true;
    }
    if (0x276A..0x276A + 1).contains(&cp) {
        return true;
    }
    if (0x276B..0x276B + 1).contains(&cp) {
        return true;
    }
    if (0x276C..0x276C + 1).contains(&cp) {
        return true;
    }
    if (0x276D..0x276D + 1).contains(&cp) {
        return true;
    }
    if (0x276E..0x276E + 1).contains(&cp) {
        return true;
    }
    if (0x276F..0x276F + 1).contains(&cp) {
        return true;
    }
    if (0x2770..0x2770 + 1).contains(&cp) {
        return true;
    }
    if (0x2771..0x2771 + 1).contains(&cp) {
        return true;
    }
    if (0x2772..0x2772 + 1).contains(&cp) {
        return true;
    }
    if (0x2773..0x2773 + 1).contains(&cp) {
        return true;
    }
    if (0x2774..0x2774 + 1).contains(&cp) {
        return true;
    }
    if (0x2775..0x2775 + 1).contains(&cp) {
        return true;
    }
    if (0x2794..0x27BF + 1).contains(&cp) {
        return true;
    }
    if (0x27C0..0x27C4 + 1).contains(&cp) {
        return true;
    }
    if (0x27C5..0x27C5 + 1).contains(&cp) {
        return true;
    }
    if (0x27C6..0x27C6 + 1).contains(&cp) {
        return true;
    }
    if (0x27C7..0x27E5 + 1).contains(&cp) {
        return true;
    }
    if (0x27E6..0x27E6 + 1).contains(&cp) {
        return true;
    }
    if (0x27E7..0x27E7 + 1).contains(&cp) {
        return true;
    }
    if (0x27E8..0x27E8 + 1).contains(&cp) {
        return true;
    }
    if (0x27E9..0x27E9 + 1).contains(&cp) {
        return true;
    }
    if (0x27EA..0x27EA + 1).contains(&cp) {
        return true;
    }
    if (0x27EB..0x27EB + 1).contains(&cp) {
        return true;
    }
    if (0x27EC..0x27EC + 1).contains(&cp) {
        return true;
    }
    if (0x27ED..0x27ED + 1).contains(&cp) {
        return true;
    }
    if (0x27EE..0x27EE + 1).contains(&cp) {
        return true;
    }
    if (0x27EF..0x27EF + 1).contains(&cp) {
        return true;
    }
    if (0x27F0..0x27FF + 1).contains(&cp) {
        return true;
    }
    if (0x2800..0x28FF + 1).contains(&cp) {
        return true;
    }
    if (0x2900..0x2982 + 1).contains(&cp) {
        return true;
    }
    if (0x2983..0x2983 + 1).contains(&cp) {
        return true;
    }
    if (0x2984..0x2984 + 1).contains(&cp) {
        return true;
    }
    if (0x2985..0x2985 + 1).contains(&cp) {
        return true;
    }
    if (0x2986..0x2986 + 1).contains(&cp) {
        return true;
    }
    if (0x2987..0x2987 + 1).contains(&cp) {
        return true;
    }
    if (0x2988..0x2988 + 1).contains(&cp) {
        return true;
    }
    if (0x2989..0x2989 + 1).contains(&cp) {
        return true;
    }
    if (0x298A..0x298A + 1).contains(&cp) {
        return true;
    }
    if (0x298B..0x298B + 1).contains(&cp) {
        return true;
    }
    if (0x298C..0x298C + 1).contains(&cp) {
        return true;
    }
    if (0x298D..0x298D + 1).contains(&cp) {
        return true;
    }
    if (0x298E..0x298E + 1).contains(&cp) {
        return true;
    }
    if (0x298F..0x298F + 1).contains(&cp) {
        return true;
    }
    if (0x2990..0x2990 + 1).contains(&cp) {
        return true;
    }
    if (0x2991..0x2991 + 1).contains(&cp) {
        return true;
    }
    if (0x2992..0x2992 + 1).contains(&cp) {
        return true;
    }
    if (0x2993..0x2993 + 1).contains(&cp) {
        return true;
    }
    if (0x2994..0x2994 + 1).contains(&cp) {
        return true;
    }
    if (0x2995..0x2995 + 1).contains(&cp) {
        return true;
    }
    if (0x2996..0x2996 + 1).contains(&cp) {
        return true;
    }
    if (0x2997..0x2997 + 1).contains(&cp) {
        return true;
    }
    if (0x2998..0x2998 + 1).contains(&cp) {
        return true;
    }
    if (0x2999..0x29D7 + 1).contains(&cp) {
        return true;
    }
    if (0x29D8..0x29D8 + 1).contains(&cp) {
        return true;
    }
    if (0x29D9..0x29D9 + 1).contains(&cp) {
        return true;
    }
    if (0x29DA..0x29DA + 1).contains(&cp) {
        return true;
    }
    if (0x29DB..0x29DB + 1).contains(&cp) {
        return true;
    }
    if (0x29DC..0x29FB + 1).contains(&cp) {
        return true;
    }
    if (0x29FC..0x29FC + 1).contains(&cp) {
        return true;
    }
    if (0x29FD..0x29FD + 1).contains(&cp) {
        return true;
    }
    if (0x29FE..0x2AFF + 1).contains(&cp) {
        return true;
    }
    if (0x2B00..0x2B2F + 1).contains(&cp) {
        return true;
    }
    if (0x2B30..0x2B44 + 1).contains(&cp) {
        return true;
    }
    if (0x2B45..0x2B46 + 1).contains(&cp) {
        return true;
    }
    if (0x2B47..0x2B4C + 1).contains(&cp) {
        return true;
    }
    if (0x2B4D..0x2B73 + 1).contains(&cp) {
        return true;
    }
    if (0x2B74..0x2B75 + 1).contains(&cp) {
        return true;
    }
    if (0x2B76..0x2B95 + 1).contains(&cp) {
        return true;
    }
    if (0x2B96..0x2B96 + 1).contains(&cp) {
        return true;
    }
    if (0x2B97..0x2BFF + 1).contains(&cp) {
        return true;
    }
    if (0x2E00..0x2E01 + 1).contains(&cp) {
        return true;
    }
    if (0x2E02..0x2E02 + 1).contains(&cp) {
        return true;
    }
    if (0x2E03..0x2E03 + 1).contains(&cp) {
        return true;
    }
    if (0x2E04..0x2E04 + 1).contains(&cp) {
        return true;
    }
    if (0x2E05..0x2E05 + 1).contains(&cp) {
        return true;
    }
    if (0x2E06..0x2E08 + 1).contains(&cp) {
        return true;
    }
    if (0x2E09..0x2E09 + 1).contains(&cp) {
        return true;
    }
    if (0x2E0A..0x2E0A + 1).contains(&cp) {
        return true;
    }
    if (0x2E0B..0x2E0B + 1).contains(&cp) {
        return true;
    }
    if (0x2E0C..0x2E0C + 1).contains(&cp) {
        return true;
    }
    if (0x2E0D..0x2E0D + 1).contains(&cp) {
        return true;
    }
    if (0x2E0E..0x2E16 + 1).contains(&cp) {
        return true;
    }
    if (0x2E17..0x2E17 + 1).contains(&cp) {
        return true;
    }
    if (0x2E18..0x2E19 + 1).contains(&cp) {
        return true;
    }
    if (0x2E1A..0x2E1A + 1).contains(&cp) {
        return true;
    }
    if (0x2E1B..0x2E1B + 1).contains(&cp) {
        return true;
    }
    if (0x2E1C..0x2E1C + 1).contains(&cp) {
        return true;
    }
    if (0x2E1D..0x2E1D + 1).contains(&cp) {
        return true;
    }
    if (0x2E1E..0x2E1F + 1).contains(&cp) {
        return true;
    }
    if (0x2E20..0x2E20 + 1).contains(&cp) {
        return true;
    }
    if (0x2E21..0x2E21 + 1).contains(&cp) {
        return true;
    }
    if (0x2E22..0x2E22 + 1).contains(&cp) {
        return true;
    }
    if (0x2E23..0x2E23 + 1).contains(&cp) {
        return true;
    }
    if (0x2E24..0x2E24 + 1).contains(&cp) {
        return true;
    }
    if (0x2E25..0x2E25 + 1).contains(&cp) {
        return true;
    }
    if (0x2E26..0x2E26 + 1).contains(&cp) {
        return true;
    }
    if (0x2E27..0x2E27 + 1).contains(&cp) {
        return true;
    }
    if (0x2E28..0x2E28 + 1).contains(&cp) {
        return true;
    }
    if (0x2E29..0x2E29 + 1).contains(&cp) {
        return true;
    }
    if (0x2E2A..0x2E2E + 1).contains(&cp) {
        return true;
    }
    if (0x2E2F..0x2E2F + 1).contains(&cp) {
        return true;
    }
    if (0x2E30..0x2E39 + 1).contains(&cp) {
        return true;
    }
    if (0x2E3A..0x2E3B + 1).contains(&cp) {
        return true;
    }
    if (0x2E3C..0x2E3F + 1).contains(&cp) {
        return true;
    }
    if (0x2E40..0x2E40 + 1).contains(&cp) {
        return true;
    }
    if (0x2E41..0x2E41 + 1).contains(&cp) {
        return true;
    }
    if (0x2E42..0x2E42 + 1).contains(&cp) {
        return true;
    }
    if (0x2E43..0x2E4F + 1).contains(&cp) {
        return true;
    }
    if (0x2E50..0x2E51 + 1).contains(&cp) {
        return true;
    }
    if (0x2E52..0x2E54 + 1).contains(&cp) {
        return true;
    }
    if (0x2E55..0x2E55 + 1).contains(&cp) {
        return true;
    }
    if (0x2E56..0x2E56 + 1).contains(&cp) {
        return true;
    }
    if (0x2E57..0x2E57 + 1).contains(&cp) {
        return true;
    }
    if (0x2E58..0x2E58 + 1).contains(&cp) {
        return true;
    }
    if (0x2E59..0x2E59 + 1).contains(&cp) {
        return true;
    }
    if (0x2E5A..0x2E5A + 1).contains(&cp) {
        return true;
    }
    if (0x2E5B..0x2E5B + 1).contains(&cp) {
        return true;
    }
    if (0x2E5C..0x2E5C + 1).contains(&cp) {
        return true;
    }
    if (0x2E5D..0x2E5D + 1).contains(&cp) {
        return true;
    }
    if (0x2E5E..0x2E7F + 1).contains(&cp) {
        return true;
    }
    if (0x3001..0x3003 + 1).contains(&cp) {
        return true;
    }
    if (0x3008..0x3008 + 1).contains(&cp) {
        return true;
    }
    if (0x3009..0x3009 + 1).contains(&cp) {
        return true;
    }
    if (0x300A..0x300A + 1).contains(&cp) {
        return true;
    }
    if (0x300B..0x300B + 1).contains(&cp) {
        return true;
    }
    if (0x300C..0x300C + 1).contains(&cp) {
        return true;
    }
    if (0x300D..0x300D + 1).contains(&cp) {
        return true;
    }
    if (0x300E..0x300E + 1).contains(&cp) {
        return true;
    }
    if (0x300F..0x300F + 1).contains(&cp) {
        return true;
    }
    if (0x3010..0x3010 + 1).contains(&cp) {
        return true;
    }
    if (0x3011..0x3011 + 1).contains(&cp) {
        return true;
    }
    if (0x3012..0x3013 + 1).contains(&cp) {
        return true;
    }
    if (0x3014..0x3014 + 1).contains(&cp) {
        return true;
    }
    if (0x3015..0x3015 + 1).contains(&cp) {
        return true;
    }
    if (0x3016..0x3016 + 1).contains(&cp) {
        return true;
    }
    if (0x3017..0x3017 + 1).contains(&cp) {
        return true;
    }
    if (0x3018..0x3018 + 1).contains(&cp) {
        return true;
    }
    if (0x3019..0x3019 + 1).contains(&cp) {
        return true;
    }
    if (0x301A..0x301A + 1).contains(&cp) {
        return true;
    }
    if (0x301B..0x301B + 1).contains(&cp) {
        return true;
    }
    if (0x301C..0x301C + 1).contains(&cp) {
        return true;
    }
    if (0x301D..0x301D + 1).contains(&cp) {
        return true;
    }
    if (0x301E..0x301F + 1).contains(&cp) {
        return true;
    }
    if (0x3020..0x3020 + 1).contains(&cp) {
        return true;
    }
    if (0x3030..0x3030 + 1).contains(&cp) {
        return true;
    }
    if (0xFD3E..0xFD3E + 1).contains(&cp) {
        return true;
    }
    if (0xFD3F..0xFD3F + 1).contains(&cp) {
        return true;
    }
    if (0xFE45..0xFE46 + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn pcm(cp: u32) -> bool {
    if (0x0600..0x0605 + 1).contains(&cp) {
        return true;
    }
    if (0x06DD..0x06DD + 1).contains(&cp) {
        return true;
    }
    if (0x070F..0x070F + 1).contains(&cp) {
        return true;
    }
    if (0x0890..0x0891 + 1).contains(&cp) {
        return true;
    }
    if (0x08E2..0x08E2 + 1).contains(&cp) {
        return true;
    }
    if (0x110BD..0x110BD + 1).contains(&cp) {
        return true;
    }
    if (0x110CD..0x110CD + 1).contains(&cp) {
        return true;
    }

    false
}

pub(crate) fn ri(cp: u32) -> bool {
    if (0x1F1E6..0x1F1FF + 1).contains(&cp) {
        return true;
    }

    false
}
