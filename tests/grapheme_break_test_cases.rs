pub const TEST_CASES: &[(&str, &[&str])] = &[
    ("\u{0020}\u{0020}", &["\u{0020}", "\u{0020}"]),
    ("\u{0020}\u{0308}\u{0020}", &["\u{0020}\u{0308}", "\u{0020}"]),
    ("\u{0020}\u{000D}", &["\u{0020}", "\u{000D}"]),
    ("\u{0020}\u{0308}\u{000D}", &["\u{0020}\u{0308}", "\u{000D}"]),
    ("\u{0020}\u{000A}", &["\u{0020}", "\u{000A}"]),
    ("\u{0020}\u{0308}\u{000A}", &["\u{0020}\u{0308}", "\u{000A}"]),
    ("\u{0020}\u{0001}", &["\u{0020}", "\u{0001}"]),
    ("\u{0020}\u{0308}\u{0001}", &["\u{0020}\u{0308}", "\u{0001}"]),
    ("\u{0020}\u{034F}", &["\u{0020}\u{034F}"]),
    ("\u{0020}\u{0308}\u{034F}", &["\u{0020}\u{0308}\u{034F}"]),
    ("\u{0020}\u{1F1E6}", &["\u{0020}", "\u{1F1E6}"]),
    ("\u{0020}\u{0308}\u{1F1E6}", &["\u{0020}\u{0308}", "\u{1F1E6}"]),
    ("\u{0020}\u{0600}", &["\u{0020}", "\u{0600}"]),
    ("\u{0020}\u{0308}\u{0600}", &["\u{0020}\u{0308}", "\u{0600}"]),
    ("\u{0020}\u{0903}", &["\u{0020}\u{0903}"]),
    ("\u{0020}\u{0308}\u{0903}", &["\u{0020}\u{0308}\u{0903}"]),
    ("\u{0020}\u{1100}", &["\u{0020}", "\u{1100}"]),
    ("\u{0020}\u{0308}\u{1100}", &["\u{0020}\u{0308}", "\u{1100}"]),
    ("\u{0020}\u{1160}", &["\u{0020}", "\u{1160}"]),
    ("\u{0020}\u{0308}\u{1160}", &["\u{0020}\u{0308}", "\u{1160}"]),
    ("\u{0020}\u{11A8}", &["\u{0020}", "\u{11A8}"]),
    ("\u{0020}\u{0308}\u{11A8}", &["\u{0020}\u{0308}", "\u{11A8}"]),
    ("\u{0020}\u{AC00}", &["\u{0020}", "\u{AC00}"]),
    ("\u{0020}\u{0308}\u{AC00}", &["\u{0020}\u{0308}", "\u{AC00}"]),
    ("\u{0020}\u{AC01}", &["\u{0020}", "\u{AC01}"]),
    ("\u{0020}\u{0308}\u{AC01}", &["\u{0020}\u{0308}", "\u{AC01}"]),
    ("\u{0020}\u{231A}", &["\u{0020}", "\u{231A}"]),
    ("\u{0020}\u{0308}\u{231A}", &["\u{0020}\u{0308}", "\u{231A}"]),
    ("\u{0020}\u{0300}", &["\u{0020}\u{0300}"]),
    ("\u{0020}\u{0308}\u{0300}", &["\u{0020}\u{0308}\u{0300}"]),
    ("\u{0020}\u{200D}", &["\u{0020}\u{200D}"]),
    ("\u{0020}\u{0308}\u{200D}", &["\u{0020}\u{0308}\u{200D}"]),
    ("\u{0020}\u{0378}", &["\u{0020}", "\u{0378}"]),
    ("\u{0020}\u{0308}\u{0378}", &["\u{0020}\u{0308}", "\u{0378}"]),
    ("\u{000D}\u{0020}", &["\u{000D}", "\u{0020}"]),
    ("\u{000D}\u{0308}\u{0020}", &["\u{000D}", "\u{0308}", "\u{0020}"]),
    ("\u{000D}\u{000D}", &["\u{000D}", "\u{000D}"]),
    ("\u{000D}\u{0308}\u{000D}", &["\u{000D}", "\u{0308}", "\u{000D}"]),
    ("\u{000D}\u{000A}", &["\u{000D}\u{000A}"]),
    ("\u{000D}\u{0308}\u{000A}", &["\u{000D}", "\u{0308}", "\u{000A}"]),
    ("\u{000D}\u{0001}", &["\u{000D}", "\u{0001}"]),
    ("\u{000D}\u{0308}\u{0001}", &["\u{000D}", "\u{0308}", "\u{0001}"]),
    ("\u{000D}\u{034F}", &["\u{000D}", "\u{034F}"]),
    ("\u{000D}\u{0308}\u{034F}", &["\u{000D}", "\u{0308}\u{034F}"]),
    ("\u{000D}\u{1F1E6}", &["\u{000D}", "\u{1F1E6}"]),
    ("\u{000D}\u{0308}\u{1F1E6}", &["\u{000D}", "\u{0308}", "\u{1F1E6}"]),
    ("\u{000D}\u{0600}", &["\u{000D}", "\u{0600}"]),
    ("\u{000D}\u{0308}\u{0600}", &["\u{000D}", "\u{0308}", "\u{0600}"]),
    ("\u{000D}\u{0903}", &["\u{000D}", "\u{0903}"]),
    ("\u{000D}\u{0308}\u{0903}", &["\u{000D}", "\u{0308}\u{0903}"]),
    ("\u{000D}\u{1100}", &["\u{000D}", "\u{1100}"]),
    ("\u{000D}\u{0308}\u{1100}", &["\u{000D}", "\u{0308}", "\u{1100}"]),
    ("\u{000D}\u{1160}", &["\u{000D}", "\u{1160}"]),
    ("\u{000D}\u{0308}\u{1160}", &["\u{000D}", "\u{0308}", "\u{1160}"]),
    ("\u{000D}\u{11A8}", &["\u{000D}", "\u{11A8}"]),
    ("\u{000D}\u{0308}\u{11A8}", &["\u{000D}", "\u{0308}", "\u{11A8}"]),
    ("\u{000D}\u{AC00}", &["\u{000D}", "\u{AC00}"]),
    ("\u{000D}\u{0308}\u{AC00}", &["\u{000D}", "\u{0308}", "\u{AC00}"]),
    ("\u{000D}\u{AC01}", &["\u{000D}", "\u{AC01}"]),
    ("\u{000D}\u{0308}\u{AC01}", &["\u{000D}", "\u{0308}", "\u{AC01}"]),
    ("\u{000D}\u{231A}", &["\u{000D}", "\u{231A}"]),
    ("\u{000D}\u{0308}\u{231A}", &["\u{000D}", "\u{0308}", "\u{231A}"]),
    ("\u{000D}\u{0300}", &["\u{000D}", "\u{0300}"]),
    ("\u{000D}\u{0308}\u{0300}", &["\u{000D}", "\u{0308}\u{0300}"]),
    ("\u{000D}\u{200D}", &["\u{000D}", "\u{200D}"]),
    ("\u{000D}\u{0308}\u{200D}", &["\u{000D}", "\u{0308}\u{200D}"]),
    ("\u{000D}\u{0378}", &["\u{000D}", "\u{0378}"]),
    ("\u{000D}\u{0308}\u{0378}", &["\u{000D}", "\u{0308}", "\u{0378}"]),
    ("\u{000A}\u{0020}", &["\u{000A}", "\u{0020}"]),
    ("\u{000A}\u{0308}\u{0020}", &["\u{000A}", "\u{0308}", "\u{0020}"]),
    ("\u{000A}\u{000D}", &["\u{000A}", "\u{000D}"]),
    ("\u{000A}\u{0308}\u{000D}", &["\u{000A}", "\u{0308}", "\u{000D}"]),
    ("\u{000A}\u{000A}", &["\u{000A}", "\u{000A}"]),
    ("\u{000A}\u{0308}\u{000A}", &["\u{000A}", "\u{0308}", "\u{000A}"]),
    ("\u{000A}\u{0001}", &["\u{000A}", "\u{0001}"]),
    ("\u{000A}\u{0308}\u{0001}", &["\u{000A}", "\u{0308}", "\u{0001}"]),
    ("\u{000A}\u{034F}", &["\u{000A}", "\u{034F}"]),
    ("\u{000A}\u{0308}\u{034F}", &["\u{000A}", "\u{0308}\u{034F}"]),
    ("\u{000A}\u{1F1E6}", &["\u{000A}", "\u{1F1E6}"]),
    ("\u{000A}\u{0308}\u{1F1E6}", &["\u{000A}", "\u{0308}", "\u{1F1E6}"]),
    ("\u{000A}\u{0600}", &["\u{000A}", "\u{0600}"]),
    ("\u{000A}\u{0308}\u{0600}", &["\u{000A}", "\u{0308}", "\u{0600}"]),
    ("\u{000A}\u{0903}", &["\u{000A}", "\u{0903}"]),
    ("\u{000A}\u{0308}\u{0903}", &["\u{000A}", "\u{0308}\u{0903}"]),
    ("\u{000A}\u{1100}", &["\u{000A}", "\u{1100}"]),
    ("\u{000A}\u{0308}\u{1100}", &["\u{000A}", "\u{0308}", "\u{1100}"]),
    ("\u{000A}\u{1160}", &["\u{000A}", "\u{1160}"]),
    ("\u{000A}\u{0308}\u{1160}", &["\u{000A}", "\u{0308}", "\u{1160}"]),
    ("\u{000A}\u{11A8}", &["\u{000A}", "\u{11A8}"]),
    ("\u{000A}\u{0308}\u{11A8}", &["\u{000A}", "\u{0308}", "\u{11A8}"]),
    ("\u{000A}\u{AC00}", &["\u{000A}", "\u{AC00}"]),
    ("\u{000A}\u{0308}\u{AC00}", &["\u{000A}", "\u{0308}", "\u{AC00}"]),
    ("\u{000A}\u{AC01}", &["\u{000A}", "\u{AC01}"]),
    ("\u{000A}\u{0308}\u{AC01}", &["\u{000A}", "\u{0308}", "\u{AC01}"]),
    ("\u{000A}\u{231A}", &["\u{000A}", "\u{231A}"]),
    ("\u{000A}\u{0308}\u{231A}", &["\u{000A}", "\u{0308}", "\u{231A}"]),
    ("\u{000A}\u{0300}", &["\u{000A}", "\u{0300}"]),
    ("\u{000A}\u{0308}\u{0300}", &["\u{000A}", "\u{0308}\u{0300}"]),
    ("\u{000A}\u{200D}", &["\u{000A}", "\u{200D}"]),
    ("\u{000A}\u{0308}\u{200D}", &["\u{000A}", "\u{0308}\u{200D}"]),
    ("\u{000A}\u{0378}", &["\u{000A}", "\u{0378}"]),
    ("\u{000A}\u{0308}\u{0378}", &["\u{000A}", "\u{0308}", "\u{0378}"]),
    ("\u{0001}\u{0020}", &["\u{0001}", "\u{0020}"]),
    ("\u{0001}\u{0308}\u{0020}", &["\u{0001}", "\u{0308}", "\u{0020}"]),
    ("\u{0001}\u{000D}", &["\u{0001}", "\u{000D}"]),
    ("\u{0001}\u{0308}\u{000D}", &["\u{0001}", "\u{0308}", "\u{000D}"]),
    ("\u{0001}\u{000A}", &["\u{0001}", "\u{000A}"]),
    ("\u{0001}\u{0308}\u{000A}", &["\u{0001}", "\u{0308}", "\u{000A}"]),
    ("\u{0001}\u{0001}", &["\u{0001}", "\u{0001}"]),
    ("\u{0001}\u{0308}\u{0001}", &["\u{0001}", "\u{0308}", "\u{0001}"]),
    ("\u{0001}\u{034F}", &["\u{0001}", "\u{034F}"]),
    ("\u{0001}\u{0308}\u{034F}", &["\u{0001}", "\u{0308}\u{034F}"]),
    ("\u{0001}\u{1F1E6}", &["\u{0001}", "\u{1F1E6}"]),
    ("\u{0001}\u{0308}\u{1F1E6}", &["\u{0001}", "\u{0308}", "\u{1F1E6}"]),
    ("\u{0001}\u{0600}", &["\u{0001}", "\u{0600}"]),
    ("\u{0001}\u{0308}\u{0600}", &["\u{0001}", "\u{0308}", "\u{0600}"]),
    ("\u{0001}\u{0903}", &["\u{0001}", "\u{0903}"]),
    ("\u{0001}\u{0308}\u{0903}", &["\u{0001}", "\u{0308}\u{0903}"]),
    ("\u{0001}\u{1100}", &["\u{0001}", "\u{1100}"]),
    ("\u{0001}\u{0308}\u{1100}", &["\u{0001}", "\u{0308}", "\u{1100}"]),
    ("\u{0001}\u{1160}", &["\u{0001}", "\u{1160}"]),
    ("\u{0001}\u{0308}\u{1160}", &["\u{0001}", "\u{0308}", "\u{1160}"]),
    ("\u{0001}\u{11A8}", &["\u{0001}", "\u{11A8}"]),
    ("\u{0001}\u{0308}\u{11A8}", &["\u{0001}", "\u{0308}", "\u{11A8}"]),
    ("\u{0001}\u{AC00}", &["\u{0001}", "\u{AC00}"]),
    ("\u{0001}\u{0308}\u{AC00}", &["\u{0001}", "\u{0308}", "\u{AC00}"]),
    ("\u{0001}\u{AC01}", &["\u{0001}", "\u{AC01}"]),
    ("\u{0001}\u{0308}\u{AC01}", &["\u{0001}", "\u{0308}", "\u{AC01}"]),
    ("\u{0001}\u{231A}", &["\u{0001}", "\u{231A}"]),
    ("\u{0001}\u{0308}\u{231A}", &["\u{0001}", "\u{0308}", "\u{231A}"]),
    ("\u{0001}\u{0300}", &["\u{0001}", "\u{0300}"]),
    ("\u{0001}\u{0308}\u{0300}", &["\u{0001}", "\u{0308}\u{0300}"]),
    ("\u{0001}\u{200D}", &["\u{0001}", "\u{200D}"]),
    ("\u{0001}\u{0308}\u{200D}", &["\u{0001}", "\u{0308}\u{200D}"]),
    ("\u{0001}\u{0378}", &["\u{0001}", "\u{0378}"]),
    ("\u{0001}\u{0308}\u{0378}", &["\u{0001}", "\u{0308}", "\u{0378}"]),
    ("\u{034F}\u{0020}", &["\u{034F}", "\u{0020}"]),
    ("\u{034F}\u{0308}\u{0020}", &["\u{034F}\u{0308}", "\u{0020}"]),
    ("\u{034F}\u{000D}", &["\u{034F}", "\u{000D}"]),
    ("\u{034F}\u{0308}\u{000D}", &["\u{034F}\u{0308}", "\u{000D}"]),
    ("\u{034F}\u{000A}", &["\u{034F}", "\u{000A}"]),
    ("\u{034F}\u{0308}\u{000A}", &["\u{034F}\u{0308}", "\u{000A}"]),
    ("\u{034F}\u{0001}", &["\u{034F}", "\u{0001}"]),
    ("\u{034F}\u{0308}\u{0001}", &["\u{034F}\u{0308}", "\u{0001}"]),
    ("\u{034F}\u{034F}", &["\u{034F}\u{034F}"]),
    ("\u{034F}\u{0308}\u{034F}", &["\u{034F}\u{0308}\u{034F}"]),
    ("\u{034F}\u{1F1E6}", &["\u{034F}", "\u{1F1E6}"]),
    ("\u{034F}\u{0308}\u{1F1E6}", &["\u{034F}\u{0308}", "\u{1F1E6}"]),
    ("\u{034F}\u{0600}", &["\u{034F}", "\u{0600}"]),
    ("\u{034F}\u{0308}\u{0600}", &["\u{034F}\u{0308}", "\u{0600}"]),
    ("\u{034F}\u{0903}", &["\u{034F}\u{0903}"]),
    ("\u{034F}\u{0308}\u{0903}", &["\u{034F}\u{0308}\u{0903}"]),
    ("\u{034F}\u{1100}", &["\u{034F}", "\u{1100}"]),
    ("\u{034F}\u{0308}\u{1100}", &["\u{034F}\u{0308}", "\u{1100}"]),
    ("\u{034F}\u{1160}", &["\u{034F}", "\u{1160}"]),
    ("\u{034F}\u{0308}\u{1160}", &["\u{034F}\u{0308}", "\u{1160}"]),
    ("\u{034F}\u{11A8}", &["\u{034F}", "\u{11A8}"]),
    ("\u{034F}\u{0308}\u{11A8}", &["\u{034F}\u{0308}", "\u{11A8}"]),
    ("\u{034F}\u{AC00}", &["\u{034F}", "\u{AC00}"]),
    ("\u{034F}\u{0308}\u{AC00}", &["\u{034F}\u{0308}", "\u{AC00}"]),
    ("\u{034F}\u{AC01}", &["\u{034F}", "\u{AC01}"]),
    ("\u{034F}\u{0308}\u{AC01}", &["\u{034F}\u{0308}", "\u{AC01}"]),
    ("\u{034F}\u{231A}", &["\u{034F}", "\u{231A}"]),
    ("\u{034F}\u{0308}\u{231A}", &["\u{034F}\u{0308}", "\u{231A}"]),
    ("\u{034F}\u{0300}", &["\u{034F}\u{0300}"]),
    ("\u{034F}\u{0308}\u{0300}", &["\u{034F}\u{0308}\u{0300}"]),
    ("\u{034F}\u{200D}", &["\u{034F}\u{200D}"]),
    ("\u{034F}\u{0308}\u{200D}", &["\u{034F}\u{0308}\u{200D}"]),
    ("\u{034F}\u{0378}", &["\u{034F}", "\u{0378}"]),
    ("\u{034F}\u{0308}\u{0378}", &["\u{034F}\u{0308}", "\u{0378}"]),
    ("\u{1F1E6}\u{0020}", &["\u{1F1E6}", "\u{0020}"]),
    ("\u{1F1E6}\u{0308}\u{0020}", &["\u{1F1E6}\u{0308}", "\u{0020}"]),
    ("\u{1F1E6}\u{000D}", &["\u{1F1E6}", "\u{000D}"]),
    ("\u{1F1E6}\u{0308}\u{000D}", &["\u{1F1E6}\u{0308}", "\u{000D}"]),
    ("\u{1F1E6}\u{000A}", &["\u{1F1E6}", "\u{000A}"]),
    ("\u{1F1E6}\u{0308}\u{000A}", &["\u{1F1E6}\u{0308}", "\u{000A}"]),
    ("\u{1F1E6}\u{0001}", &["\u{1F1E6}", "\u{0001}"]),
    ("\u{1F1E6}\u{0308}\u{0001}", &["\u{1F1E6}\u{0308}", "\u{0001}"]),
    ("\u{1F1E6}\u{034F}", &["\u{1F1E6}\u{034F}"]),
    ("\u{1F1E6}\u{0308}\u{034F}", &["\u{1F1E6}\u{0308}\u{034F}"]),
    ("\u{1F1E6}\u{1F1E6}", &["\u{1F1E6}\u{1F1E6}"]),
    ("\u{1F1E6}\u{0308}\u{1F1E6}", &["\u{1F1E6}\u{0308}", "\u{1F1E6}"]),
    ("\u{1F1E6}\u{0600}", &["\u{1F1E6}", "\u{0600}"]),
    ("\u{1F1E6}\u{0308}\u{0600}", &["\u{1F1E6}\u{0308}", "\u{0600}"]),
    ("\u{1F1E6}\u{0903}", &["\u{1F1E6}\u{0903}"]),
    ("\u{1F1E6}\u{0308}\u{0903}", &["\u{1F1E6}\u{0308}\u{0903}"]),
    ("\u{1F1E6}\u{1100}", &["\u{1F1E6}", "\u{1100}"]),
    ("\u{1F1E6}\u{0308}\u{1100}", &["\u{1F1E6}\u{0308}", "\u{1100}"]),
    ("\u{1F1E6}\u{1160}", &["\u{1F1E6}", "\u{1160}"]),
    ("\u{1F1E6}\u{0308}\u{1160}", &["\u{1F1E6}\u{0308}", "\u{1160}"]),
    ("\u{1F1E6}\u{11A8}", &["\u{1F1E6}", "\u{11A8}"]),
    ("\u{1F1E6}\u{0308}\u{11A8}", &["\u{1F1E6}\u{0308}", "\u{11A8}"]),
    ("\u{1F1E6}\u{AC00}", &["\u{1F1E6}", "\u{AC00}"]),
    ("\u{1F1E6}\u{0308}\u{AC00}", &["\u{1F1E6}\u{0308}", "\u{AC00}"]),
    ("\u{1F1E6}\u{AC01}", &["\u{1F1E6}", "\u{AC01}"]),
    ("\u{1F1E6}\u{0308}\u{AC01}", &["\u{1F1E6}\u{0308}", "\u{AC01}"]),
    ("\u{1F1E6}\u{231A}", &["\u{1F1E6}", "\u{231A}"]),
    ("\u{1F1E6}\u{0308}\u{231A}", &["\u{1F1E6}\u{0308}", "\u{231A}"]),
    ("\u{1F1E6}\u{0300}", &["\u{1F1E6}\u{0300}"]),
    ("\u{1F1E6}\u{0308}\u{0300}", &["\u{1F1E6}\u{0308}\u{0300}"]),
    ("\u{1F1E6}\u{200D}", &["\u{1F1E6}\u{200D}"]),
    ("\u{1F1E6}\u{0308}\u{200D}", &["\u{1F1E6}\u{0308}\u{200D}"]),
    ("\u{1F1E6}\u{0378}", &["\u{1F1E6}", "\u{0378}"]),
    ("\u{1F1E6}\u{0308}\u{0378}", &["\u{1F1E6}\u{0308}", "\u{0378}"]),
    ("\u{0600}\u{0020}", &["\u{0600}\u{0020}"]),
    ("\u{0600}\u{0308}\u{0020}", &["\u{0600}\u{0308}", "\u{0020}"]),
    ("\u{0600}\u{000D}", &["\u{0600}", "\u{000D}"]),
    ("\u{0600}\u{0308}\u{000D}", &["\u{0600}\u{0308}", "\u{000D}"]),
    ("\u{0600}\u{000A}", &["\u{0600}", "\u{000A}"]),
    ("\u{0600}\u{0308}\u{000A}", &["\u{0600}\u{0308}", "\u{000A}"]),
    ("\u{0600}\u{0001}", &["\u{0600}", "\u{0001}"]),
    ("\u{0600}\u{0308}\u{0001}", &["\u{0600}\u{0308}", "\u{0001}"]),
    ("\u{0600}\u{034F}", &["\u{0600}\u{034F}"]),
    ("\u{0600}\u{0308}\u{034F}", &["\u{0600}\u{0308}\u{034F}"]),
    ("\u{0600}\u{1F1E6}", &["\u{0600}\u{1F1E6}"]),
    ("\u{0600}\u{0308}\u{1F1E6}", &["\u{0600}\u{0308}", "\u{1F1E6}"]),
    ("\u{0600}\u{0600}", &["\u{0600}\u{0600}"]),
    ("\u{0600}\u{0308}\u{0600}", &["\u{0600}\u{0308}", "\u{0600}"]),
    ("\u{0600}\u{0903}", &["\u{0600}\u{0903}"]),
    ("\u{0600}\u{0308}\u{0903}", &["\u{0600}\u{0308}\u{0903}"]),
    ("\u{0600}\u{1100}", &["\u{0600}\u{1100}"]),
    ("\u{0600}\u{0308}\u{1100}", &["\u{0600}\u{0308}", "\u{1100}"]),
    ("\u{0600}\u{1160}", &["\u{0600}\u{1160}"]),
    ("\u{0600}\u{0308}\u{1160}", &["\u{0600}\u{0308}", "\u{1160}"]),
    ("\u{0600}\u{11A8}", &["\u{0600}\u{11A8}"]),
    ("\u{0600}\u{0308}\u{11A8}", &["\u{0600}\u{0308}", "\u{11A8}"]),
    ("\u{0600}\u{AC00}", &["\u{0600}\u{AC00}"]),
    ("\u{0600}\u{0308}\u{AC00}", &["\u{0600}\u{0308}", "\u{AC00}"]),
    ("\u{0600}\u{AC01}", &["\u{0600}\u{AC01}"]),
    ("\u{0600}\u{0308}\u{AC01}", &["\u{0600}\u{0308}", "\u{AC01}"]),
    ("\u{0600}\u{231A}", &["\u{0600}\u{231A}"]),
    ("\u{0600}\u{0308}\u{231A}", &["\u{0600}\u{0308}", "\u{231A}"]),
    ("\u{0600}\u{0300}", &["\u{0600}\u{0300}"]),
    ("\u{0600}\u{0308}\u{0300}", &["\u{0600}\u{0308}\u{0300}"]),
    ("\u{0600}\u{200D}", &["\u{0600}\u{200D}"]),
    ("\u{0600}\u{0308}\u{200D}", &["\u{0600}\u{0308}\u{200D}"]),
    ("\u{0600}\u{0378}", &["\u{0600}\u{0378}"]),
    ("\u{0600}\u{0308}\u{0378}", &["\u{0600}\u{0308}", "\u{0378}"]),
    ("\u{0903}\u{0020}", &["\u{0903}", "\u{0020}"]),
    ("\u{0903}\u{0308}\u{0020}", &["\u{0903}\u{0308}", "\u{0020}"]),
    ("\u{0903}\u{000D}", &["\u{0903}", "\u{000D}"]),
    ("\u{0903}\u{0308}\u{000D}", &["\u{0903}\u{0308}", "\u{000D}"]),
    ("\u{0903}\u{000A}", &["\u{0903}", "\u{000A}"]),
    ("\u{0903}\u{0308}\u{000A}", &["\u{0903}\u{0308}", "\u{000A}"]),
    ("\u{0903}\u{0001}", &["\u{0903}", "\u{0001}"]),
    ("\u{0903}\u{0308}\u{0001}", &["\u{0903}\u{0308}", "\u{0001}"]),
    ("\u{0903}\u{034F}", &["\u{0903}\u{034F}"]),
    ("\u{0903}\u{0308}\u{034F}", &["\u{0903}\u{0308}\u{034F}"]),
    ("\u{0903}\u{1F1E6}", &["\u{0903}", "\u{1F1E6}"]),
    ("\u{0903}\u{0308}\u{1F1E6}", &["\u{0903}\u{0308}", "\u{1F1E6}"]),
    ("\u{0903}\u{0600}", &["\u{0903}", "\u{0600}"]),
    ("\u{0903}\u{0308}\u{0600}", &["\u{0903}\u{0308}", "\u{0600}"]),
    ("\u{0903}\u{0903}", &["\u{0903}\u{0903}"]),
    ("\u{0903}\u{0308}\u{0903}", &["\u{0903}\u{0308}\u{0903}"]),
    ("\u{0903}\u{1100}", &["\u{0903}", "\u{1100}"]),
    ("\u{0903}\u{0308}\u{1100}", &["\u{0903}\u{0308}", "\u{1100}"]),
    ("\u{0903}\u{1160}", &["\u{0903}", "\u{1160}"]),
    ("\u{0903}\u{0308}\u{1160}", &["\u{0903}\u{0308}", "\u{1160}"]),
    ("\u{0903}\u{11A8}", &["\u{0903}", "\u{11A8}"]),
    ("\u{0903}\u{0308}\u{11A8}", &["\u{0903}\u{0308}", "\u{11A8}"]),
    ("\u{0903}\u{AC00}", &["\u{0903}", "\u{AC00}"]),
    ("\u{0903}\u{0308}\u{AC00}", &["\u{0903}\u{0308}", "\u{AC00}"]),
    ("\u{0903}\u{AC01}", &["\u{0903}", "\u{AC01}"]),
    ("\u{0903}\u{0308}\u{AC01}", &["\u{0903}\u{0308}", "\u{AC01}"]),
    ("\u{0903}\u{231A}", &["\u{0903}", "\u{231A}"]),
    ("\u{0903}\u{0308}\u{231A}", &["\u{0903}\u{0308}", "\u{231A}"]),
    ("\u{0903}\u{0300}", &["\u{0903}\u{0300}"]),
    ("\u{0903}\u{0308}\u{0300}", &["\u{0903}\u{0308}\u{0300}"]),
    ("\u{0903}\u{200D}", &["\u{0903}\u{200D}"]),
    ("\u{0903}\u{0308}\u{200D}", &["\u{0903}\u{0308}\u{200D}"]),
    ("\u{0903}\u{0378}", &["\u{0903}", "\u{0378}"]),
    ("\u{0903}\u{0308}\u{0378}", &["\u{0903}\u{0308}", "\u{0378}"]),
    ("\u{1100}\u{0020}", &["\u{1100}", "\u{0020}"]),
    ("\u{1100}\u{0308}\u{0020}", &["\u{1100}\u{0308}", "\u{0020}"]),
    ("\u{1100}\u{000D}", &["\u{1100}", "\u{000D}"]),
    ("\u{1100}\u{0308}\u{000D}", &["\u{1100}\u{0308}", "\u{000D}"]),
    ("\u{1100}\u{000A}", &["\u{1100}", "\u{000A}"]),
    ("\u{1100}\u{0308}\u{000A}", &["\u{1100}\u{0308}", "\u{000A}"]),
    ("\u{1100}\u{0001}", &["\u{1100}", "\u{0001}"]),
    ("\u{1100}\u{0308}\u{0001}", &["\u{1100}\u{0308}", "\u{0001}"]),
    ("\u{1100}\u{034F}", &["\u{1100}\u{034F}"]),
    ("\u{1100}\u{0308}\u{034F}", &["\u{1100}\u{0308}\u{034F}"]),
    ("\u{1100}\u{1F1E6}", &["\u{1100}", "\u{1F1E6}"]),
    ("\u{1100}\u{0308}\u{1F1E6}", &["\u{1100}\u{0308}", "\u{1F1E6}"]),
    ("\u{1100}\u{0600}", &["\u{1100}", "\u{0600}"]),
    ("\u{1100}\u{0308}\u{0600}", &["\u{1100}\u{0308}", "\u{0600}"]),
    ("\u{1100}\u{0903}", &["\u{1100}\u{0903}"]),
    ("\u{1100}\u{0308}\u{0903}", &["\u{1100}\u{0308}\u{0903}"]),
    ("\u{1100}\u{1100}", &["\u{1100}\u{1100}"]),
    ("\u{1100}\u{0308}\u{1100}", &["\u{1100}\u{0308}", "\u{1100}"]),
    ("\u{1100}\u{1160}", &["\u{1100}\u{1160}"]),
    ("\u{1100}\u{0308}\u{1160}", &["\u{1100}\u{0308}", "\u{1160}"]),
    ("\u{1100}\u{11A8}", &["\u{1100}", "\u{11A8}"]),
    ("\u{1100}\u{0308}\u{11A8}", &["\u{1100}\u{0308}", "\u{11A8}"]),
    ("\u{1100}\u{AC00}", &["\u{1100}\u{AC00}"]),
    ("\u{1100}\u{0308}\u{AC00}", &["\u{1100}\u{0308}", "\u{AC00}"]),
    ("\u{1100}\u{AC01}", &["\u{1100}\u{AC01}"]),
    ("\u{1100}\u{0308}\u{AC01}", &["\u{1100}\u{0308}", "\u{AC01}"]),
    ("\u{1100}\u{231A}", &["\u{1100}", "\u{231A}"]),
    ("\u{1100}\u{0308}\u{231A}", &["\u{1100}\u{0308}", "\u{231A}"]),
    ("\u{1100}\u{0300}", &["\u{1100}\u{0300}"]),
    ("\u{1100}\u{0308}\u{0300}", &["\u{1100}\u{0308}\u{0300}"]),
    ("\u{1100}\u{200D}", &["\u{1100}\u{200D}"]),
    ("\u{1100}\u{0308}\u{200D}", &["\u{1100}\u{0308}\u{200D}"]),
    ("\u{1100}\u{0378}", &["\u{1100}", "\u{0378}"]),
    ("\u{1100}\u{0308}\u{0378}", &["\u{1100}\u{0308}", "\u{0378}"]),
    ("\u{1160}\u{0020}", &["\u{1160}", "\u{0020}"]),
    ("\u{1160}\u{0308}\u{0020}", &["\u{1160}\u{0308}", "\u{0020}"]),
    ("\u{1160}\u{000D}", &["\u{1160}", "\u{000D}"]),
    ("\u{1160}\u{0308}\u{000D}", &["\u{1160}\u{0308}", "\u{000D}"]),
    ("\u{1160}\u{000A}", &["\u{1160}", "\u{000A}"]),
    ("\u{1160}\u{0308}\u{000A}", &["\u{1160}\u{0308}", "\u{000A}"]),
    ("\u{1160}\u{0001}", &["\u{1160}", "\u{0001}"]),
    ("\u{1160}\u{0308}\u{0001}", &["\u{1160}\u{0308}", "\u{0001}"]),
    ("\u{1160}\u{034F}", &["\u{1160}\u{034F}"]),
    ("\u{1160}\u{0308}\u{034F}", &["\u{1160}\u{0308}\u{034F}"]),
    ("\u{1160}\u{1F1E6}", &["\u{1160}", "\u{1F1E6}"]),
    ("\u{1160}\u{0308}\u{1F1E6}", &["\u{1160}\u{0308}", "\u{1F1E6}"]),
    ("\u{1160}\u{0600}", &["\u{1160}", "\u{0600}"]),
    ("\u{1160}\u{0308}\u{0600}", &["\u{1160}\u{0308}", "\u{0600}"]),
    ("\u{1160}\u{0903}", &["\u{1160}\u{0903}"]),
    ("\u{1160}\u{0308}\u{0903}", &["\u{1160}\u{0308}\u{0903}"]),
    ("\u{1160}\u{1100}", &["\u{1160}", "\u{1100}"]),
    ("\u{1160}\u{0308}\u{1100}", &["\u{1160}\u{0308}", "\u{1100}"]),
    ("\u{1160}\u{1160}", &["\u{1160}\u{1160}"]),
    ("\u{1160}\u{0308}\u{1160}", &["\u{1160}\u{0308}", "\u{1160}"]),
    ("\u{1160}\u{11A8}", &["\u{1160}\u{11A8}"]),
    ("\u{1160}\u{0308}\u{11A8}", &["\u{1160}\u{0308}", "\u{11A8}"]),
    ("\u{1160}\u{AC00}", &["\u{1160}", "\u{AC00}"]),
    ("\u{1160}\u{0308}\u{AC00}", &["\u{1160}\u{0308}", "\u{AC00}"]),
    ("\u{1160}\u{AC01}", &["\u{1160}", "\u{AC01}"]),
    ("\u{1160}\u{0308}\u{AC01}", &["\u{1160}\u{0308}", "\u{AC01}"]),
    ("\u{1160}\u{231A}", &["\u{1160}", "\u{231A}"]),
    ("\u{1160}\u{0308}\u{231A}", &["\u{1160}\u{0308}", "\u{231A}"]),
    ("\u{1160}\u{0300}", &["\u{1160}\u{0300}"]),
    ("\u{1160}\u{0308}\u{0300}", &["\u{1160}\u{0308}\u{0300}"]),
    ("\u{1160}\u{200D}", &["\u{1160}\u{200D}"]),
    ("\u{1160}\u{0308}\u{200D}", &["\u{1160}\u{0308}\u{200D}"]),
    ("\u{1160}\u{0378}", &["\u{1160}", "\u{0378}"]),
    ("\u{1160}\u{0308}\u{0378}", &["\u{1160}\u{0308}", "\u{0378}"]),
    ("\u{11A8}\u{0020}", &["\u{11A8}", "\u{0020}"]),
    ("\u{11A8}\u{0308}\u{0020}", &["\u{11A8}\u{0308}", "\u{0020}"]),
    ("\u{11A8}\u{000D}", &["\u{11A8}", "\u{000D}"]),
    ("\u{11A8}\u{0308}\u{000D}", &["\u{11A8}\u{0308}", "\u{000D}"]),
    ("\u{11A8}\u{000A}", &["\u{11A8}", "\u{000A}"]),
    ("\u{11A8}\u{0308}\u{000A}", &["\u{11A8}\u{0308}", "\u{000A}"]),
    ("\u{11A8}\u{0001}", &["\u{11A8}", "\u{0001}"]),
    ("\u{11A8}\u{0308}\u{0001}", &["\u{11A8}\u{0308}", "\u{0001}"]),
    ("\u{11A8}\u{034F}", &["\u{11A8}\u{034F}"]),
    ("\u{11A8}\u{0308}\u{034F}", &["\u{11A8}\u{0308}\u{034F}"]),
    ("\u{11A8}\u{1F1E6}", &["\u{11A8}", "\u{1F1E6}"]),
    ("\u{11A8}\u{0308}\u{1F1E6}", &["\u{11A8}\u{0308}", "\u{1F1E6}"]),
    ("\u{11A8}\u{0600}", &["\u{11A8}", "\u{0600}"]),
    ("\u{11A8}\u{0308}\u{0600}", &["\u{11A8}\u{0308}", "\u{0600}"]),
    ("\u{11A8}\u{0903}", &["\u{11A8}\u{0903}"]),
    ("\u{11A8}\u{0308}\u{0903}", &["\u{11A8}\u{0308}\u{0903}"]),
    ("\u{11A8}\u{1100}", &["\u{11A8}", "\u{1100}"]),
    ("\u{11A8}\u{0308}\u{1100}", &["\u{11A8}\u{0308}", "\u{1100}"]),
    ("\u{11A8}\u{1160}", &["\u{11A8}", "\u{1160}"]),
    ("\u{11A8}\u{0308}\u{1160}", &["\u{11A8}\u{0308}", "\u{1160}"]),
    ("\u{11A8}\u{11A8}", &["\u{11A8}\u{11A8}"]),
    ("\u{11A8}\u{0308}\u{11A8}", &["\u{11A8}\u{0308}", "\u{11A8}"]),
    ("\u{11A8}\u{AC00}", &["\u{11A8}", "\u{AC00}"]),
    ("\u{11A8}\u{0308}\u{AC00}", &["\u{11A8}\u{0308}", "\u{AC00}"]),
    ("\u{11A8}\u{AC01}", &["\u{11A8}", "\u{AC01}"]),
    ("\u{11A8}\u{0308}\u{AC01}", &["\u{11A8}\u{0308}", "\u{AC01}"]),
    ("\u{11A8}\u{231A}", &["\u{11A8}", "\u{231A}"]),
    ("\u{11A8}\u{0308}\u{231A}", &["\u{11A8}\u{0308}", "\u{231A}"]),
    ("\u{11A8}\u{0300}", &["\u{11A8}\u{0300}"]),
    ("\u{11A8}\u{0308}\u{0300}", &["\u{11A8}\u{0308}\u{0300}"]),
    ("\u{11A8}\u{200D}", &["\u{11A8}\u{200D}"]),
    ("\u{11A8}\u{0308}\u{200D}", &["\u{11A8}\u{0308}\u{200D}"]),
    ("\u{11A8}\u{0378}", &["\u{11A8}", "\u{0378}"]),
    ("\u{11A8}\u{0308}\u{0378}", &["\u{11A8}\u{0308}", "\u{0378}"]),
    ("\u{AC00}\u{0020}", &["\u{AC00}", "\u{0020}"]),
    ("\u{AC00}\u{0308}\u{0020}", &["\u{AC00}\u{0308}", "\u{0020}"]),
    ("\u{AC00}\u{000D}", &["\u{AC00}", "\u{000D}"]),
    ("\u{AC00}\u{0308}\u{000D}", &["\u{AC00}\u{0308}", "\u{000D}"]),
    ("\u{AC00}\u{000A}", &["\u{AC00}", "\u{000A}"]),
    ("\u{AC00}\u{0308}\u{000A}", &["\u{AC00}\u{0308}", "\u{000A}"]),
    ("\u{AC00}\u{0001}", &["\u{AC00}", "\u{0001}"]),
    ("\u{AC00}\u{0308}\u{0001}", &["\u{AC00}\u{0308}", "\u{0001}"]),
    ("\u{AC00}\u{034F}", &["\u{AC00}\u{034F}"]),
    ("\u{AC00}\u{0308}\u{034F}", &["\u{AC00}\u{0308}\u{034F}"]),
    ("\u{AC00}\u{1F1E6}", &["\u{AC00}", "\u{1F1E6}"]),
    ("\u{AC00}\u{0308}\u{1F1E6}", &["\u{AC00}\u{0308}", "\u{1F1E6}"]),
    ("\u{AC00}\u{0600}", &["\u{AC00}", "\u{0600}"]),
    ("\u{AC00}\u{0308}\u{0600}", &["\u{AC00}\u{0308}", "\u{0600}"]),
    ("\u{AC00}\u{0903}", &["\u{AC00}\u{0903}"]),
    ("\u{AC00}\u{0308}\u{0903}", &["\u{AC00}\u{0308}\u{0903}"]),
    ("\u{AC00}\u{1100}", &["\u{AC00}", "\u{1100}"]),
    ("\u{AC00}\u{0308}\u{1100}", &["\u{AC00}\u{0308}", "\u{1100}"]),
    ("\u{AC00}\u{1160}", &["\u{AC00}\u{1160}"]),
    ("\u{AC00}\u{0308}\u{1160}", &["\u{AC00}\u{0308}", "\u{1160}"]),
    ("\u{AC00}\u{11A8}", &["\u{AC00}\u{11A8}"]),
    ("\u{AC00}\u{0308}\u{11A8}", &["\u{AC00}\u{0308}", "\u{11A8}"]),
    ("\u{AC00}\u{AC00}", &["\u{AC00}", "\u{AC00}"]),
    ("\u{AC00}\u{0308}\u{AC00}", &["\u{AC00}\u{0308}", "\u{AC00}"]),
    ("\u{AC00}\u{AC01}", &["\u{AC00}", "\u{AC01}"]),
    ("\u{AC00}\u{0308}\u{AC01}", &["\u{AC00}\u{0308}", "\u{AC01}"]),
    ("\u{AC00}\u{231A}", &["\u{AC00}", "\u{231A}"]),
    ("\u{AC00}\u{0308}\u{231A}", &["\u{AC00}\u{0308}", "\u{231A}"]),
    ("\u{AC00}\u{0300}", &["\u{AC00}\u{0300}"]),
    ("\u{AC00}\u{0308}\u{0300}", &["\u{AC00}\u{0308}\u{0300}"]),
    ("\u{AC00}\u{200D}", &["\u{AC00}\u{200D}"]),
    ("\u{AC00}\u{0308}\u{200D}", &["\u{AC00}\u{0308}\u{200D}"]),
    ("\u{AC00}\u{0378}", &["\u{AC00}", "\u{0378}"]),
    ("\u{AC00}\u{0308}\u{0378}", &["\u{AC00}\u{0308}", "\u{0378}"]),
    ("\u{AC01}\u{0020}", &["\u{AC01}", "\u{0020}"]),
    ("\u{AC01}\u{0308}\u{0020}", &["\u{AC01}\u{0308}", "\u{0020}"]),
    ("\u{AC01}\u{000D}", &["\u{AC01}", "\u{000D}"]),
    ("\u{AC01}\u{0308}\u{000D}", &["\u{AC01}\u{0308}", "\u{000D}"]),
    ("\u{AC01}\u{000A}", &["\u{AC01}", "\u{000A}"]),
    ("\u{AC01}\u{0308}\u{000A}", &["\u{AC01}\u{0308}", "\u{000A}"]),
    ("\u{AC01}\u{0001}", &["\u{AC01}", "\u{0001}"]),
    ("\u{AC01}\u{0308}\u{0001}", &["\u{AC01}\u{0308}", "\u{0001}"]),
    ("\u{AC01}\u{034F}", &["\u{AC01}\u{034F}"]),
    ("\u{AC01}\u{0308}\u{034F}", &["\u{AC01}\u{0308}\u{034F}"]),
    ("\u{AC01}\u{1F1E6}", &["\u{AC01}", "\u{1F1E6}"]),
    ("\u{AC01}\u{0308}\u{1F1E6}", &["\u{AC01}\u{0308}", "\u{1F1E6}"]),
    ("\u{AC01}\u{0600}", &["\u{AC01}", "\u{0600}"]),
    ("\u{AC01}\u{0308}\u{0600}", &["\u{AC01}\u{0308}", "\u{0600}"]),
    ("\u{AC01}\u{0903}", &["\u{AC01}\u{0903}"]),
    ("\u{AC01}\u{0308}\u{0903}", &["\u{AC01}\u{0308}\u{0903}"]),
    ("\u{AC01}\u{1100}", &["\u{AC01}", "\u{1100}"]),
    ("\u{AC01}\u{0308}\u{1100}", &["\u{AC01}\u{0308}", "\u{1100}"]),
    ("\u{AC01}\u{1160}", &["\u{AC01}", "\u{1160}"]),
    ("\u{AC01}\u{0308}\u{1160}", &["\u{AC01}\u{0308}", "\u{1160}"]),
    ("\u{AC01}\u{11A8}", &["\u{AC01}\u{11A8}"]),
    ("\u{AC01}\u{0308}\u{11A8}", &["\u{AC01}\u{0308}", "\u{11A8}"]),
    ("\u{AC01}\u{AC00}", &["\u{AC01}", "\u{AC00}"]),
    ("\u{AC01}\u{0308}\u{AC00}", &["\u{AC01}\u{0308}", "\u{AC00}"]),
    ("\u{AC01}\u{AC01}", &["\u{AC01}", "\u{AC01}"]),
    ("\u{AC01}\u{0308}\u{AC01}", &["\u{AC01}\u{0308}", "\u{AC01}"]),
    ("\u{AC01}\u{231A}", &["\u{AC01}", "\u{231A}"]),
    ("\u{AC01}\u{0308}\u{231A}", &["\u{AC01}\u{0308}", "\u{231A}"]),
    ("\u{AC01}\u{0300}", &["\u{AC01}\u{0300}"]),
    ("\u{AC01}\u{0308}\u{0300}", &["\u{AC01}\u{0308}\u{0300}"]),
    ("\u{AC01}\u{200D}", &["\u{AC01}\u{200D}"]),
    ("\u{AC01}\u{0308}\u{200D}", &["\u{AC01}\u{0308}\u{200D}"]),
    ("\u{AC01}\u{0378}", &["\u{AC01}", "\u{0378}"]),
    ("\u{AC01}\u{0308}\u{0378}", &["\u{AC01}\u{0308}", "\u{0378}"]),
    ("\u{231A}\u{0020}", &["\u{231A}", "\u{0020}"]),
    ("\u{231A}\u{0308}\u{0020}", &["\u{231A}\u{0308}", "\u{0020}"]),
    ("\u{231A}\u{000D}", &["\u{231A}", "\u{000D}"]),
    ("\u{231A}\u{0308}\u{000D}", &["\u{231A}\u{0308}", "\u{000D}"]),
    ("\u{231A}\u{000A}", &["\u{231A}", "\u{000A}"]),
    ("\u{231A}\u{0308}\u{000A}", &["\u{231A}\u{0308}", "\u{000A}"]),
    ("\u{231A}\u{0001}", &["\u{231A}", "\u{0001}"]),
    ("\u{231A}\u{0308}\u{0001}", &["\u{231A}\u{0308}", "\u{0001}"]),
    ("\u{231A}\u{034F}", &["\u{231A}\u{034F}"]),
    ("\u{231A}\u{0308}\u{034F}", &["\u{231A}\u{0308}\u{034F}"]),
    ("\u{231A}\u{1F1E6}", &["\u{231A}", "\u{1F1E6}"]),
    ("\u{231A}\u{0308}\u{1F1E6}", &["\u{231A}\u{0308}", "\u{1F1E6}"]),
    ("\u{231A}\u{0600}", &["\u{231A}", "\u{0600}"]),
    ("\u{231A}\u{0308}\u{0600}", &["\u{231A}\u{0308}", "\u{0600}"]),
    ("\u{231A}\u{0903}", &["\u{231A}\u{0903}"]),
    ("\u{231A}\u{0308}\u{0903}", &["\u{231A}\u{0308}\u{0903}"]),
    ("\u{231A}\u{1100}", &["\u{231A}", "\u{1100}"]),
    ("\u{231A}\u{0308}\u{1100}", &["\u{231A}\u{0308}", "\u{1100}"]),
    ("\u{231A}\u{1160}", &["\u{231A}", "\u{1160}"]),
    ("\u{231A}\u{0308}\u{1160}", &["\u{231A}\u{0308}", "\u{1160}"]),
    ("\u{231A}\u{11A8}", &["\u{231A}", "\u{11A8}"]),
    ("\u{231A}\u{0308}\u{11A8}", &["\u{231A}\u{0308}", "\u{11A8}"]),
    ("\u{231A}\u{AC00}", &["\u{231A}", "\u{AC00}"]),
    ("\u{231A}\u{0308}\u{AC00}", &["\u{231A}\u{0308}", "\u{AC00}"]),
    ("\u{231A}\u{AC01}", &["\u{231A}", "\u{AC01}"]),
    ("\u{231A}\u{0308}\u{AC01}", &["\u{231A}\u{0308}", "\u{AC01}"]),
    ("\u{231A}\u{231A}", &["\u{231A}", "\u{231A}"]),
    ("\u{231A}\u{0308}\u{231A}", &["\u{231A}\u{0308}", "\u{231A}"]),
    ("\u{231A}\u{0300}", &["\u{231A}\u{0300}"]),
    ("\u{231A}\u{0308}\u{0300}", &["\u{231A}\u{0308}\u{0300}"]),
    ("\u{231A}\u{200D}", &["\u{231A}\u{200D}"]),
    ("\u{231A}\u{0308}\u{200D}", &["\u{231A}\u{0308}\u{200D}"]),
    ("\u{231A}\u{0378}", &["\u{231A}", "\u{0378}"]),
    ("\u{231A}\u{0308}\u{0378}", &["\u{231A}\u{0308}", "\u{0378}"]),
    ("\u{0300}\u{0020}", &["\u{0300}", "\u{0020}"]),
    ("\u{0300}\u{0308}\u{0020}", &["\u{0300}\u{0308}", "\u{0020}"]),
    ("\u{0300}\u{000D}", &["\u{0300}", "\u{000D}"]),
    ("\u{0300}\u{0308}\u{000D}", &["\u{0300}\u{0308}", "\u{000D}"]),
    ("\u{0300}\u{000A}", &["\u{0300}", "\u{000A}"]),
    ("\u{0300}\u{0308}\u{000A}", &["\u{0300}\u{0308}", "\u{000A}"]),
    ("\u{0300}\u{0001}", &["\u{0300}", "\u{0001}"]),
    ("\u{0300}\u{0308}\u{0001}", &["\u{0300}\u{0308}", "\u{0001}"]),
    ("\u{0300}\u{034F}", &["\u{0300}\u{034F}"]),
    ("\u{0300}\u{0308}\u{034F}", &["\u{0300}\u{0308}\u{034F}"]),
    ("\u{0300}\u{1F1E6}", &["\u{0300}", "\u{1F1E6}"]),
    ("\u{0300}\u{0308}\u{1F1E6}", &["\u{0300}\u{0308}", "\u{1F1E6}"]),
    ("\u{0300}\u{0600}", &["\u{0300}", "\u{0600}"]),
    ("\u{0300}\u{0308}\u{0600}", &["\u{0300}\u{0308}", "\u{0600}"]),
    ("\u{0300}\u{0903}", &["\u{0300}\u{0903}"]),
    ("\u{0300}\u{0308}\u{0903}", &["\u{0300}\u{0308}\u{0903}"]),
    ("\u{0300}\u{1100}", &["\u{0300}", "\u{1100}"]),
    ("\u{0300}\u{0308}\u{1100}", &["\u{0300}\u{0308}", "\u{1100}"]),
    ("\u{0300}\u{1160}", &["\u{0300}", "\u{1160}"]),
    ("\u{0300}\u{0308}\u{1160}", &["\u{0300}\u{0308}", "\u{1160}"]),
    ("\u{0300}\u{11A8}", &["\u{0300}", "\u{11A8}"]),
    ("\u{0300}\u{0308}\u{11A8}", &["\u{0300}\u{0308}", "\u{11A8}"]),
    ("\u{0300}\u{AC00}", &["\u{0300}", "\u{AC00}"]),
    ("\u{0300}\u{0308}\u{AC00}", &["\u{0300}\u{0308}", "\u{AC00}"]),
    ("\u{0300}\u{AC01}", &["\u{0300}", "\u{AC01}"]),
    ("\u{0300}\u{0308}\u{AC01}", &["\u{0300}\u{0308}", "\u{AC01}"]),
    ("\u{0300}\u{231A}", &["\u{0300}", "\u{231A}"]),
    ("\u{0300}\u{0308}\u{231A}", &["\u{0300}\u{0308}", "\u{231A}"]),
    ("\u{0300}\u{0300}", &["\u{0300}\u{0300}"]),
    ("\u{0300}\u{0308}\u{0300}", &["\u{0300}\u{0308}\u{0300}"]),
    ("\u{0300}\u{200D}", &["\u{0300}\u{200D}"]),
    ("\u{0300}\u{0308}\u{200D}", &["\u{0300}\u{0308}\u{200D}"]),
    ("\u{0300}\u{0378}", &["\u{0300}", "\u{0378}"]),
    ("\u{0300}\u{0308}\u{0378}", &["\u{0300}\u{0308}", "\u{0378}"]),
    ("\u{200D}\u{0020}", &["\u{200D}", "\u{0020}"]),
    ("\u{200D}\u{0308}\u{0020}", &["\u{200D}\u{0308}", "\u{0020}"]),
    ("\u{200D}\u{000D}", &["\u{200D}", "\u{000D}"]),
    ("\u{200D}\u{0308}\u{000D}", &["\u{200D}\u{0308}", "\u{000D}"]),
    ("\u{200D}\u{000A}", &["\u{200D}", "\u{000A}"]),
    ("\u{200D}\u{0308}\u{000A}", &["\u{200D}\u{0308}", "\u{000A}"]),
    ("\u{200D}\u{0001}", &["\u{200D}", "\u{0001}"]),
    ("\u{200D}\u{0308}\u{0001}", &["\u{200D}\u{0308}", "\u{0001}"]),
    ("\u{200D}\u{034F}", &["\u{200D}\u{034F}"]),
    ("\u{200D}\u{0308}\u{034F}", &["\u{200D}\u{0308}\u{034F}"]),
    ("\u{200D}\u{1F1E6}", &["\u{200D}", "\u{1F1E6}"]),
    ("\u{200D}\u{0308}\u{1F1E6}", &["\u{200D}\u{0308}", "\u{1F1E6}"]),
    ("\u{200D}\u{0600}", &["\u{200D}", "\u{0600}"]),
    ("\u{200D}\u{0308}\u{0600}", &["\u{200D}\u{0308}", "\u{0600}"]),
    ("\u{200D}\u{0903}", &["\u{200D}\u{0903}"]),
    ("\u{200D}\u{0308}\u{0903}", &["\u{200D}\u{0308}\u{0903}"]),
    ("\u{200D}\u{1100}", &["\u{200D}", "\u{1100}"]),
    ("\u{200D}\u{0308}\u{1100}", &["\u{200D}\u{0308}", "\u{1100}"]),
    ("\u{200D}\u{1160}", &["\u{200D}", "\u{1160}"]),
    ("\u{200D}\u{0308}\u{1160}", &["\u{200D}\u{0308}", "\u{1160}"]),
    ("\u{200D}\u{11A8}", &["\u{200D}", "\u{11A8}"]),
    ("\u{200D}\u{0308}\u{11A8}", &["\u{200D}\u{0308}", "\u{11A8}"]),
    ("\u{200D}\u{AC00}", &["\u{200D}", "\u{AC00}"]),
    ("\u{200D}\u{0308}\u{AC00}", &["\u{200D}\u{0308}", "\u{AC00}"]),
    ("\u{200D}\u{AC01}", &["\u{200D}", "\u{AC01}"]),
    ("\u{200D}\u{0308}\u{AC01}", &["\u{200D}\u{0308}", "\u{AC01}"]),
    ("\u{200D}\u{231A}", &["\u{200D}", "\u{231A}"]),
    ("\u{200D}\u{0308}\u{231A}", &["\u{200D}\u{0308}", "\u{231A}"]),
    ("\u{200D}\u{0300}", &["\u{200D}\u{0300}"]),
    ("\u{200D}\u{0308}\u{0300}", &["\u{200D}\u{0308}\u{0300}"]),
    ("\u{200D}\u{200D}", &["\u{200D}\u{200D}"]),
    ("\u{200D}\u{0308}\u{200D}", &["\u{200D}\u{0308}\u{200D}"]),
    ("\u{200D}\u{0378}", &["\u{200D}", "\u{0378}"]),
    ("\u{200D}\u{0308}\u{0378}", &["\u{200D}\u{0308}", "\u{0378}"]),
    ("\u{0378}\u{0020}", &["\u{0378}", "\u{0020}"]),
    ("\u{0378}\u{0308}\u{0020}", &["\u{0378}\u{0308}", "\u{0020}"]),
    ("\u{0378}\u{000D}", &["\u{0378}", "\u{000D}"]),
    ("\u{0378}\u{0308}\u{000D}", &["\u{0378}\u{0308}", "\u{000D}"]),
    ("\u{0378}\u{000A}", &["\u{0378}", "\u{000A}"]),
    ("\u{0378}\u{0308}\u{000A}", &["\u{0378}\u{0308}", "\u{000A}"]),
    ("\u{0378}\u{0001}", &["\u{0378}", "\u{0001}"]),
    ("\u{0378}\u{0308}\u{0001}", &["\u{0378}\u{0308}", "\u{0001}"]),
    ("\u{0378}\u{034F}", &["\u{0378}\u{034F}"]),
    ("\u{0378}\u{0308}\u{034F}", &["\u{0378}\u{0308}\u{034F}"]),
    ("\u{0378}\u{1F1E6}", &["\u{0378}", "\u{1F1E6}"]),
    ("\u{0378}\u{0308}\u{1F1E6}", &["\u{0378}\u{0308}", "\u{1F1E6}"]),
    ("\u{0378}\u{0600}", &["\u{0378}", "\u{0600}"]),
    ("\u{0378}\u{0308}\u{0600}", &["\u{0378}\u{0308}", "\u{0600}"]),
    ("\u{0378}\u{0903}", &["\u{0378}\u{0903}"]),
    ("\u{0378}\u{0308}\u{0903}", &["\u{0378}\u{0308}\u{0903}"]),
    ("\u{0378}\u{1100}", &["\u{0378}", "\u{1100}"]),
    ("\u{0378}\u{0308}\u{1100}", &["\u{0378}\u{0308}", "\u{1100}"]),
    ("\u{0378}\u{1160}", &["\u{0378}", "\u{1160}"]),
    ("\u{0378}\u{0308}\u{1160}", &["\u{0378}\u{0308}", "\u{1160}"]),
    ("\u{0378}\u{11A8}", &["\u{0378}", "\u{11A8}"]),
    ("\u{0378}\u{0308}\u{11A8}", &["\u{0378}\u{0308}", "\u{11A8}"]),
    ("\u{0378}\u{AC00}", &["\u{0378}", "\u{AC00}"]),
    ("\u{0378}\u{0308}\u{AC00}", &["\u{0378}\u{0308}", "\u{AC00}"]),
    ("\u{0378}\u{AC01}", &["\u{0378}", "\u{AC01}"]),
    ("\u{0378}\u{0308}\u{AC01}", &["\u{0378}\u{0308}", "\u{AC01}"]),
    ("\u{0378}\u{231A}", &["\u{0378}", "\u{231A}"]),
    ("\u{0378}\u{0308}\u{231A}", &["\u{0378}\u{0308}", "\u{231A}"]),
    ("\u{0378}\u{0300}", &["\u{0378}\u{0300}"]),
    ("\u{0378}\u{0308}\u{0300}", &["\u{0378}\u{0308}\u{0300}"]),
    ("\u{0378}\u{200D}", &["\u{0378}\u{200D}"]),
    ("\u{0378}\u{0308}\u{200D}", &["\u{0378}\u{0308}\u{200D}"]),
    ("\u{0378}\u{0378}", &["\u{0378}", "\u{0378}"]),
    ("\u{0378}\u{0308}\u{0378}", &["\u{0378}\u{0308}", "\u{0378}"]),
    ("\u{000D}\u{000A}\u{0061}\u{000A}\u{0308}", &["\u{000D}\u{000A}", "\u{0061}", "\u{000A}", "\u{0308}"]),
    ("\u{0061}\u{0308}", &["\u{0061}\u{0308}"]),
    ("\u{0020}\u{200D}\u{0646}", &["\u{0020}\u{200D}", "\u{0646}"]),
    ("\u{0646}\u{200D}\u{0020}", &["\u{0646}\u{200D}", "\u{0020}"]),
    ("\u{AC00}\u{11A8}\u{1100}", &["\u{AC00}\u{11A8}", "\u{1100}"]),
    ("\u{AC01}\u{11A8}\u{1100}", &["\u{AC01}\u{11A8}", "\u{1100}"]),
    ("\u{1F1E6}\u{1F1E7}\u{1F1E8}\u{0062}", &["\u{1F1E6}\u{1F1E7}", "\u{1F1E8}", "\u{0062}"]),
    ("\u{0061}\u{1F1E6}\u{1F1E7}\u{1F1E8}\u{0062}", &["\u{0061}", "\u{1F1E6}\u{1F1E7}", "\u{1F1E8}", "\u{0062}"]),
    ("\u{0061}\u{1F1E6}\u{1F1E7}\u{200D}\u{1F1E8}\u{0062}", &["\u{0061}", "\u{1F1E6}\u{1F1E7}\u{200D}", "\u{1F1E8}", "\u{0062}"]),
    ("\u{0061}\u{1F1E6}\u{200D}\u{1F1E7}\u{1F1E8}\u{0062}", &["\u{0061}", "\u{1F1E6}\u{200D}", "\u{1F1E7}\u{1F1E8}", "\u{0062}"]),
    ("\u{0061}\u{1F1E6}\u{1F1E7}\u{1F1E8}\u{1F1E9}\u{0062}", &["\u{0061}", "\u{1F1E6}\u{1F1E7}", "\u{1F1E8}\u{1F1E9}", "\u{0062}"]),
    ("\u{0061}\u{200D}", &["\u{0061}\u{200D}"]),
    ("\u{0061}\u{0308}\u{0062}", &["\u{0061}\u{0308}", "\u{0062}"]),
    ("\u{0061}\u{0903}\u{0062}", &["\u{0061}\u{0903}", "\u{0062}"]),
    ("\u{0061}\u{0600}\u{0062}", &["\u{0061}", "\u{0600}\u{0062}"]),
    ("\u{1F476}\u{1F3FF}\u{1F476}", &["\u{1F476}\u{1F3FF}", "\u{1F476}"]),
    ("\u{0061}\u{1F3FF}\u{1F476}", &["\u{0061}\u{1F3FF}", "\u{1F476}"]),
    ("\u{0061}\u{1F3FF}\u{1F476}\u{200D}\u{1F6D1}", &["\u{0061}\u{1F3FF}", "\u{1F476}\u{200D}\u{1F6D1}"]),
    ("\u{1F476}\u{1F3FF}\u{0308}\u{200D}\u{1F476}\u{1F3FF}", &["\u{1F476}\u{1F3FF}\u{0308}\u{200D}\u{1F476}\u{1F3FF}"]),
    ("\u{1F6D1}\u{200D}\u{1F6D1}", &["\u{1F6D1}\u{200D}\u{1F6D1}"]),
    ("\u{0061}\u{200D}\u{1F6D1}", &["\u{0061}\u{200D}", "\u{1F6D1}"]),
    ("\u{2701}\u{200D}\u{2701}", &["\u{2701}\u{200D}\u{2701}"]),
    ("\u{0061}\u{200D}\u{2701}", &["\u{0061}\u{200D}", "\u{2701}"]),
];