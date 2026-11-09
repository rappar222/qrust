use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentKind {
    Url,
    Text,
    Wifi,
    VCard,
    Email,
    Phone,
    Sms,
    Geo,
    Crypto,
}

impl ContentKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Url => "🌐 URL",
            Self::Text => "📝 Text",
            Self::Wifi => "📶 Wi-Fi",
            Self::VCard => "📇 Contact",
            Self::Email => "📧 Email",
            Self::Phone => "📞 Phone",
            Self::Sms => "💬 SMS",
            Self::Geo => "📍 Location",
            Self::Crypto => "🪙 Crypto",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiEncryption {
    Wpa,
    Wep,
    None,
}

#[derive(Debug, Clone)]
pub struct WifiData {
    pub ssid: String,
    pub password: String,
    pub encryption: WifiEncryption,
    pub hidden: bool,
}

impl Default for WifiData {
    fn default() -> Self {
        Self {
            ssid: "MyHomeWifi".to_string(),
            password: "SuperSecretPassword123".to_string(),
            encryption: WifiEncryption::Wpa,
            hidden: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VCardData {
    pub first_name: String,
    pub last_name: String,
    pub organization: String,
    pub job_title: String,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub note: String,
}

impl Default for VCardData {
    fn default() -> Self {
        Self {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            organization: "Acme Corp".to_string(),
            job_title: "Software Engineer".to_string(),
            phone: "+1 555-0199".to_string(),
            email: "john.doe@example.com".to_string(),
            website: "https://example.com".to_string(),
            note: "Looking forward to connecting!".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmailData {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Default for EmailData {
    fn default() -> Self {
        Self {
            to: "contact@example.com".to_string(),
            subject: "Inquiry regarding services".to_string(),
            body: "Hello,\n\nI would like to inquire about your services.\n\nBest regards,".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SmsData {
    pub phone: String,
    pub message: String,
}

impl Default for SmsData {
    fn default() -> Self {
        Self {
            phone: "+1 555-0199".to_string(),
            message: "Hello from QR Craft!".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeoData {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for GeoData {
    fn default() -> Self {
        Self {
            latitude: 48.8584,
            longitude: 2.2945, // Eiffel Tower
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoCurrency {
    Bitcoin,
    Ethereum,
    Solana,
    Usdt,
}

#[derive(Debug, Clone)]
pub struct CryptoData {
    pub currency: CryptoCurrency,
    pub address: String,
    pub amount: String,
}

impl Default for CryptoData {
    fn default() -> Self {
        Self {
            currency: CryptoCurrency::Bitcoin,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            amount: "0.005".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcLevelSetting {
    Low,
    Medium,
    Quartile,
    High,
}

impl EcLevelSetting {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Low => "L (~7%)",
            Self::Medium => "M (~15%)",
            Self::Quartile => "Q (~25%)",
            Self::High => "H (~30%)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Low => "Low: Restores ~7% of damaged code. Smallest QR size.",
            Self::Medium => "Medium: Restores ~15% of damaged code. Recommended default.",
            Self::Quartile => "Quartile: Restores ~25% of damaged code. Good for rough surfaces.",
            Self::High => "High: Restores ~30% of damaged code. Best durability & custom logos.",
        }
    }

    pub fn to_qrcode_level(&self) -> qrcode::EcLevel {
        match self {
            Self::Low => qrcode::EcLevel::L,
            Self::Medium => qrcode::EcLevel::M,
            Self::Quartile => qrcode::EcLevel::Q,
            Self::High => qrcode::EcLevel::H,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleShape {
    Square,
    Rounded,
    Circle,
    SoftSquare,
}

impl ModuleShape {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Square => "⬛ Sharp Square",
            Self::Rounded => "🔲 Rounded",
            Self::Circle => "⚪ Circle / Dot",
            Self::SoftSquare => "🔘 Smooth Squircle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EyeShape {
    Unified,
    RoundedEye,
    CircleEye,
}

impl EyeShape {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Unified => "Match Modules",
            Self::RoundedEye => "Rounded Outer",
            Self::CircleEye => "Circular Target",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleConfig {
    pub fg_color: Color32,
    pub bg_color: Color32,
    pub transparent_bg: bool,
    pub module_shape: ModuleShape,
    pub eye_shape: EyeShape,
    pub margin: usize,
    pub ec_level: EcLevelSetting,
    pub export_size: u32,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            fg_color: Color32::from_rgb(26, 32, 44),      // Deep Charcoal
            bg_color: Color32::from_rgb(255, 255, 255),  // Pure White
            transparent_bg: false,
            module_shape: ModuleShape::Square,
            eye_shape: EyeShape::Unified,
            margin: 4,
            ec_level: EcLevelSetting::Medium,
            export_size: 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalettePreset {
    pub name: &'static str,
    pub fg: Color32,
    pub bg: Color32,
}

pub const PALETTES: &[PalettePreset] = &[
    PalettePreset {
        name: "Classic",
        fg: Color32::from_rgb(15, 23, 42),       // Slate 900
        bg: Color32::from_rgb(255, 255, 255),
    },
    PalettePreset {
        name: "Ocean Navy",
        fg: Color32::from_rgb(29, 78, 216),      // Blue 700
        bg: Color32::from_rgb(239, 246, 255),    // Blue 50
    },
    PalettePreset {
        name: "Forest",
        fg: Color32::from_rgb(6, 95, 70),        // Emerald 800
        bg: Color32::from_rgb(236, 253, 245),    // Emerald 50
    },
    PalettePreset {
        name: "Sunset",
        fg: Color32::from_rgb(190, 24, 93),      // Rose 700
        bg: Color32::from_rgb(255, 241, 242),    // Rose 50
    },
    PalettePreset {
        name: "Cyber Violet",
        fg: Color32::from_rgb(109, 40, 217),     // Purple 700
        bg: Color32::from_rgb(245, 243, 255),    // Purple 50
    },
    PalettePreset {
        name: "Amber Warm",
        fg: Color32::from_rgb(180, 83, 9),       // Amber 700
        bg: Color32::from_rgb(254, 252, 232),    // Amber 50
    },
    PalettePreset {
        name: "Dark Neon",
        fg: Color32::from_rgb(56, 189, 248),     // Sky 400
        bg: Color32::from_rgb(15, 23, 42),       // Slate 900
    },
];

#[derive(Debug, Clone)]
pub struct ToastNotification {
    pub message: String,
    pub is_error: bool,
    pub expiry_time: f64,
}
