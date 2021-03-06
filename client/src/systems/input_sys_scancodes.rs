use strum_macros::EnumString;

#[derive(EnumString)]
pub enum ScancodeNames {
    A = sdl2::keyboard::Scancode::A as isize,
    B = sdl2::keyboard::Scancode::B as isize,
    C = sdl2::keyboard::Scancode::C as isize,
    D = sdl2::keyboard::Scancode::D as isize,
    E = sdl2::keyboard::Scancode::E as isize,
    F = sdl2::keyboard::Scancode::F as isize,
    G = sdl2::keyboard::Scancode::G as isize,
    H = sdl2::keyboard::Scancode::H as isize,
    I = sdl2::keyboard::Scancode::I as isize,
    J = sdl2::keyboard::Scancode::J as isize,
    K = sdl2::keyboard::Scancode::K as isize,
    L = sdl2::keyboard::Scancode::L as isize,
    M = sdl2::keyboard::Scancode::M as isize,
    N = sdl2::keyboard::Scancode::N as isize,
    O = sdl2::keyboard::Scancode::O as isize,
    P = sdl2::keyboard::Scancode::P as isize,
    Q = sdl2::keyboard::Scancode::Q as isize,
    R = sdl2::keyboard::Scancode::R as isize,
    S = sdl2::keyboard::Scancode::S as isize,
    T = sdl2::keyboard::Scancode::T as isize,
    U = sdl2::keyboard::Scancode::U as isize,
    V = sdl2::keyboard::Scancode::V as isize,
    W = sdl2::keyboard::Scancode::W as isize,
    X = sdl2::keyboard::Scancode::X as isize,
    Y = sdl2::keyboard::Scancode::Y as isize,
    Z = sdl2::keyboard::Scancode::Z as isize,
    Num1 = sdl2::keyboard::Scancode::Num1 as isize,
    Num2 = sdl2::keyboard::Scancode::Num2 as isize,
    Num3 = sdl2::keyboard::Scancode::Num3 as isize,
    Num4 = sdl2::keyboard::Scancode::Num4 as isize,
    Num5 = sdl2::keyboard::Scancode::Num5 as isize,
    Num6 = sdl2::keyboard::Scancode::Num6 as isize,
    Num7 = sdl2::keyboard::Scancode::Num7 as isize,
    Num8 = sdl2::keyboard::Scancode::Num8 as isize,
    Num9 = sdl2::keyboard::Scancode::Num9 as isize,
    Num0 = sdl2::keyboard::Scancode::Num0 as isize,
    Return = sdl2::keyboard::Scancode::Return as isize,
    Escape = sdl2::keyboard::Scancode::Escape as isize,
    Backspace = sdl2::keyboard::Scancode::Backspace as isize,
    Tab = sdl2::keyboard::Scancode::Tab as isize,
    Space = sdl2::keyboard::Scancode::Space as isize,
    Minus = sdl2::keyboard::Scancode::Minus as isize,
    Equals = sdl2::keyboard::Scancode::Equals as isize,
    LeftBracket = sdl2::keyboard::Scancode::LeftBracket as isize,
    RightBracket = sdl2::keyboard::Scancode::RightBracket as isize,
    Backslash = sdl2::keyboard::Scancode::Backslash as isize,
    NonUsHash = sdl2::keyboard::Scancode::NonUsHash as isize,
    Semicolon = sdl2::keyboard::Scancode::Semicolon as isize,
    Apostrophe = sdl2::keyboard::Scancode::Apostrophe as isize,
    Grave = sdl2::keyboard::Scancode::Grave as isize,
    Comma = sdl2::keyboard::Scancode::Comma as isize,
    Period = sdl2::keyboard::Scancode::Period as isize,
    Slash = sdl2::keyboard::Scancode::Slash as isize,
    CapsLock = sdl2::keyboard::Scancode::CapsLock as isize,
    F1 = sdl2::keyboard::Scancode::F1 as isize,
    F2 = sdl2::keyboard::Scancode::F2 as isize,
    F3 = sdl2::keyboard::Scancode::F3 as isize,
    F4 = sdl2::keyboard::Scancode::F4 as isize,
    F5 = sdl2::keyboard::Scancode::F5 as isize,
    F6 = sdl2::keyboard::Scancode::F6 as isize,
    F7 = sdl2::keyboard::Scancode::F7 as isize,
    F8 = sdl2::keyboard::Scancode::F8 as isize,
    F9 = sdl2::keyboard::Scancode::F9 as isize,
    F10 = sdl2::keyboard::Scancode::F10 as isize,
    F11 = sdl2::keyboard::Scancode::F11 as isize,
    F12 = sdl2::keyboard::Scancode::F12 as isize,
    PrintScreen = sdl2::keyboard::Scancode::PrintScreen as isize,
    ScrollLock = sdl2::keyboard::Scancode::ScrollLock as isize,
    Pause = sdl2::keyboard::Scancode::Pause as isize,
    Insert = sdl2::keyboard::Scancode::Insert as isize,
    Home = sdl2::keyboard::Scancode::Home as isize,
    PageUp = sdl2::keyboard::Scancode::PageUp as isize,
    Delete = sdl2::keyboard::Scancode::Delete as isize,
    End = sdl2::keyboard::Scancode::End as isize,
    PageDown = sdl2::keyboard::Scancode::PageDown as isize,
    Right = sdl2::keyboard::Scancode::Right as isize,
    Left = sdl2::keyboard::Scancode::Left as isize,
    Down = sdl2::keyboard::Scancode::Down as isize,
    Up = sdl2::keyboard::Scancode::Up as isize,
    NumLockClear = sdl2::keyboard::Scancode::NumLockClear as isize,
    KpDivide = sdl2::keyboard::Scancode::KpDivide as isize,
    KpMultiply = sdl2::keyboard::Scancode::KpMultiply as isize,
    KpMinus = sdl2::keyboard::Scancode::KpMinus as isize,
    KpPlus = sdl2::keyboard::Scancode::KpPlus as isize,
    KpEnter = sdl2::keyboard::Scancode::KpEnter as isize,
    Kp1 = sdl2::keyboard::Scancode::Kp1 as isize,
    Kp2 = sdl2::keyboard::Scancode::Kp2 as isize,
    Kp3 = sdl2::keyboard::Scancode::Kp3 as isize,
    Kp4 = sdl2::keyboard::Scancode::Kp4 as isize,
    Kp5 = sdl2::keyboard::Scancode::Kp5 as isize,
    Kp6 = sdl2::keyboard::Scancode::Kp6 as isize,
    Kp7 = sdl2::keyboard::Scancode::Kp7 as isize,
    Kp8 = sdl2::keyboard::Scancode::Kp8 as isize,
    Kp9 = sdl2::keyboard::Scancode::Kp9 as isize,
    Kp0 = sdl2::keyboard::Scancode::Kp0 as isize,
    KpPeriod = sdl2::keyboard::Scancode::KpPeriod as isize,
    NonUsBackslash = sdl2::keyboard::Scancode::NonUsBackslash as isize,
    Application = sdl2::keyboard::Scancode::Application as isize,
    Power = sdl2::keyboard::Scancode::Power as isize,
    KpEquals = sdl2::keyboard::Scancode::KpEquals as isize,
    F13 = sdl2::keyboard::Scancode::F13 as isize,
    F14 = sdl2::keyboard::Scancode::F14 as isize,
    F15 = sdl2::keyboard::Scancode::F15 as isize,
    F16 = sdl2::keyboard::Scancode::F16 as isize,
    F17 = sdl2::keyboard::Scancode::F17 as isize,
    F18 = sdl2::keyboard::Scancode::F18 as isize,
    F19 = sdl2::keyboard::Scancode::F19 as isize,
    F20 = sdl2::keyboard::Scancode::F20 as isize,
    F21 = sdl2::keyboard::Scancode::F21 as isize,
    F22 = sdl2::keyboard::Scancode::F22 as isize,
    F23 = sdl2::keyboard::Scancode::F23 as isize,
    F24 = sdl2::keyboard::Scancode::F24 as isize,
    Execute = sdl2::keyboard::Scancode::Execute as isize,
    Help = sdl2::keyboard::Scancode::Help as isize,
    Menu = sdl2::keyboard::Scancode::Menu as isize,
    Select = sdl2::keyboard::Scancode::Select as isize,
    Stop = sdl2::keyboard::Scancode::Stop as isize,
    Again = sdl2::keyboard::Scancode::Again as isize,
    Undo = sdl2::keyboard::Scancode::Undo as isize,
    Cut = sdl2::keyboard::Scancode::Cut as isize,
    Copy = sdl2::keyboard::Scancode::Copy as isize,
    Paste = sdl2::keyboard::Scancode::Paste as isize,
    Find = sdl2::keyboard::Scancode::Find as isize,
    Mute = sdl2::keyboard::Scancode::Mute as isize,
    VolumeUp = sdl2::keyboard::Scancode::VolumeUp as isize,
    VolumeDown = sdl2::keyboard::Scancode::VolumeDown as isize,
    KpComma = sdl2::keyboard::Scancode::KpComma as isize,
    KpEqualsAS400 = sdl2::keyboard::Scancode::KpEqualsAS400 as isize,
    International1 = sdl2::keyboard::Scancode::International1 as isize,
    International2 = sdl2::keyboard::Scancode::International2 as isize,
    International3 = sdl2::keyboard::Scancode::International3 as isize,
    International4 = sdl2::keyboard::Scancode::International4 as isize,
    International5 = sdl2::keyboard::Scancode::International5 as isize,
    International6 = sdl2::keyboard::Scancode::International6 as isize,
    International7 = sdl2::keyboard::Scancode::International7 as isize,
    International8 = sdl2::keyboard::Scancode::International8 as isize,
    International9 = sdl2::keyboard::Scancode::International9 as isize,
    Lang1 = sdl2::keyboard::Scancode::Lang1 as isize,
    Lang2 = sdl2::keyboard::Scancode::Lang2 as isize,
    Lang3 = sdl2::keyboard::Scancode::Lang3 as isize,
    Lang4 = sdl2::keyboard::Scancode::Lang4 as isize,
    Lang5 = sdl2::keyboard::Scancode::Lang5 as isize,
    Lang6 = sdl2::keyboard::Scancode::Lang6 as isize,
    Lang7 = sdl2::keyboard::Scancode::Lang7 as isize,
    Lang8 = sdl2::keyboard::Scancode::Lang8 as isize,
    Lang9 = sdl2::keyboard::Scancode::Lang9 as isize,
    AltErase = sdl2::keyboard::Scancode::AltErase as isize,
    SysReq = sdl2::keyboard::Scancode::SysReq as isize,
    Cancel = sdl2::keyboard::Scancode::Cancel as isize,
    Clear = sdl2::keyboard::Scancode::Clear as isize,
    Prior = sdl2::keyboard::Scancode::Prior as isize,
    Return2 = sdl2::keyboard::Scancode::Return2 as isize,
    Separator = sdl2::keyboard::Scancode::Separator as isize,
    Out = sdl2::keyboard::Scancode::Out as isize,
    Oper = sdl2::keyboard::Scancode::Oper as isize,
    ClearAgain = sdl2::keyboard::Scancode::ClearAgain as isize,
    CrSel = sdl2::keyboard::Scancode::CrSel as isize,
    ExSel = sdl2::keyboard::Scancode::ExSel as isize,
    Kp00 = sdl2::keyboard::Scancode::Kp00 as isize,
    Kp000 = sdl2::keyboard::Scancode::Kp000 as isize,
    ThousandsSeparator = sdl2::keyboard::Scancode::ThousandsSeparator as isize,
    DecimalSeparator = sdl2::keyboard::Scancode::DecimalSeparator as isize,
    CurrencyUnit = sdl2::keyboard::Scancode::CurrencyUnit as isize,
    CurrencySubUnit = sdl2::keyboard::Scancode::CurrencySubUnit as isize,
    KpLeftParen = sdl2::keyboard::Scancode::KpLeftParen as isize,
    KpRightParen = sdl2::keyboard::Scancode::KpRightParen as isize,
    KpLeftBrace = sdl2::keyboard::Scancode::KpLeftBrace as isize,
    KpRightBrace = sdl2::keyboard::Scancode::KpRightBrace as isize,
    KpTab = sdl2::keyboard::Scancode::KpTab as isize,
    KpBackspace = sdl2::keyboard::Scancode::KpBackspace as isize,
    KpA = sdl2::keyboard::Scancode::KpA as isize,
    KpB = sdl2::keyboard::Scancode::KpB as isize,
    KpC = sdl2::keyboard::Scancode::KpC as isize,
    KpD = sdl2::keyboard::Scancode::KpD as isize,
    KpE = sdl2::keyboard::Scancode::KpE as isize,
    KpF = sdl2::keyboard::Scancode::KpF as isize,
    KpXor = sdl2::keyboard::Scancode::KpXor as isize,
    KpPower = sdl2::keyboard::Scancode::KpPower as isize,
    KpPercent = sdl2::keyboard::Scancode::KpPercent as isize,
    KpLess = sdl2::keyboard::Scancode::KpLess as isize,
    KpGreater = sdl2::keyboard::Scancode::KpGreater as isize,
    KpAmpersand = sdl2::keyboard::Scancode::KpAmpersand as isize,
    KpDblAmpersand = sdl2::keyboard::Scancode::KpDblAmpersand as isize,
    KpVerticalBar = sdl2::keyboard::Scancode::KpVerticalBar as isize,
    KpDblVerticalBar = sdl2::keyboard::Scancode::KpDblVerticalBar as isize,
    KpColon = sdl2::keyboard::Scancode::KpColon as isize,
    KpHash = sdl2::keyboard::Scancode::KpHash as isize,
    KpSpace = sdl2::keyboard::Scancode::KpSpace as isize,
    KpAt = sdl2::keyboard::Scancode::KpAt as isize,
    KpExclam = sdl2::keyboard::Scancode::KpExclam as isize,
    KpMemStore = sdl2::keyboard::Scancode::KpMemStore as isize,
    KpMemRecall = sdl2::keyboard::Scancode::KpMemRecall as isize,
    KpMemClear = sdl2::keyboard::Scancode::KpMemClear as isize,
    KpMemAdd = sdl2::keyboard::Scancode::KpMemAdd as isize,
    KpMemSubtract = sdl2::keyboard::Scancode::KpMemSubtract as isize,
    KpMemMultiply = sdl2::keyboard::Scancode::KpMemMultiply as isize,
    KpMemDivide = sdl2::keyboard::Scancode::KpMemDivide as isize,
    KpPlusMinus = sdl2::keyboard::Scancode::KpPlusMinus as isize,
    KpClear = sdl2::keyboard::Scancode::KpClear as isize,
    KpClearEntry = sdl2::keyboard::Scancode::KpClearEntry as isize,
    KpBinary = sdl2::keyboard::Scancode::KpBinary as isize,
    KpOctal = sdl2::keyboard::Scancode::KpOctal as isize,
    KpDecimal = sdl2::keyboard::Scancode::KpDecimal as isize,
    KpHexadecimal = sdl2::keyboard::Scancode::KpHexadecimal as isize,
    LCtrl = sdl2::keyboard::Scancode::LCtrl as isize,
    LShift = sdl2::keyboard::Scancode::LShift as isize,
    LAlt = sdl2::keyboard::Scancode::LAlt as isize,
    LGui = sdl2::keyboard::Scancode::LGui as isize,
    RCtrl = sdl2::keyboard::Scancode::RCtrl as isize,
    RShift = sdl2::keyboard::Scancode::RShift as isize,
    RAlt = sdl2::keyboard::Scancode::RAlt as isize,
    RGui = sdl2::keyboard::Scancode::RGui as isize,
    Mode = sdl2::keyboard::Scancode::Mode as isize,
    AudioNext = sdl2::keyboard::Scancode::AudioNext as isize,
    AudioPrev = sdl2::keyboard::Scancode::AudioPrev as isize,
    AudioStop = sdl2::keyboard::Scancode::AudioStop as isize,
    AudioPlay = sdl2::keyboard::Scancode::AudioPlay as isize,
    AudioMute = sdl2::keyboard::Scancode::AudioMute as isize,
    MediaSelect = sdl2::keyboard::Scancode::MediaSelect as isize,
    Www = sdl2::keyboard::Scancode::Www as isize,
    Mail = sdl2::keyboard::Scancode::Mail as isize,
    Calculator = sdl2::keyboard::Scancode::Calculator as isize,
    Computer = sdl2::keyboard::Scancode::Computer as isize,
    AcSearch = sdl2::keyboard::Scancode::AcSearch as isize,
    AcHome = sdl2::keyboard::Scancode::AcHome as isize,
    AcBack = sdl2::keyboard::Scancode::AcBack as isize,
    AcForward = sdl2::keyboard::Scancode::AcForward as isize,
    AcStop = sdl2::keyboard::Scancode::AcStop as isize,
    AcRefresh = sdl2::keyboard::Scancode::AcRefresh as isize,
    AcBookmarks = sdl2::keyboard::Scancode::AcBookmarks as isize,
    BrightnessDown = sdl2::keyboard::Scancode::BrightnessDown as isize,
    BrightnessUp = sdl2::keyboard::Scancode::BrightnessUp as isize,
    DisplaySwitch = sdl2::keyboard::Scancode::DisplaySwitch as isize,
    KbdIllumToggle = sdl2::keyboard::Scancode::KbdIllumToggle as isize,
    KbdIllumDown = sdl2::keyboard::Scancode::KbdIllumDown as isize,
    KbdIllumUp = sdl2::keyboard::Scancode::KbdIllumUp as isize,
    Eject = sdl2::keyboard::Scancode::Eject as isize,
    Sleep = sdl2::keyboard::Scancode::Sleep as isize,
    App1 = sdl2::keyboard::Scancode::App1 as isize,
    App2 = sdl2::keyboard::Scancode::App2 as isize,
    Num = sdl2::keyboard::Scancode::Num as isize,
}
