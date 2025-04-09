#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareRegister {
    // Joypad
    P1 = 0xFF00,
    
    // Serial
    SB = 0xFF01,
    SC = 0xFF02,
    
    // Timer
    DIV = 0xFF04,
    TIMA = 0xFF05,
    TMA = 0xFF06,
    TAC = 0xFF07,
    
    // Interrupts
    IF = 0xFF0F,
    IE = 0xFFFF,
    
    // Sound Channel 1
    NR10 = 0xFF10,
    NR11 = 0xFF11,
    NR12 = 0xFF12,
    NR13 = 0xFF13,
    NR14 = 0xFF14,
    
    // Sound Channel 2
    NR21 = 0xFF16,
    NR22 = 0xFF17,
    NR23 = 0xFF18,
    NR24 = 0xFF19,
    
    // Sound Channel 3
    NR30 = 0xFF1A,
    NR31 = 0xFF1B,
    NR32 = 0xFF1C,
    NR33 = 0xFF1D,
    NR34 = 0xFF1E,
    
    // Sound Channel 4
    NR41 = 0xFF20,
    NR42 = 0xFF21,
    NR43 = 0xFF22,
    NR44 = 0xFF23,
    
    // Sound Control
    NR50 = 0xFF24,
    NR51 = 0xFF25,
    NR52 = 0xFF26,
    
    // Wave RAM
    WaveRAM = 0xFF30, // Range: 0xFF30-0xFF3F
    
    // LCD
    LCDC = 0xFF40,
    STAT = 0xFF41,
    SCY = 0xFF42,
    SCX = 0xFF43,
    LY = 0xFF44,
    LYC = 0xFF45,
    
    // DMA
    DMA = 0xFF46,
    
    // Palettes
    BGP = 0xFF47,
    OBP0 = 0xFF48,
    OBP1 = 0xFF49,
    
    // Window
    WY = 0xFF4A,
    WX = 0xFF4B,
    
    // CGB Registers
    KEY1 = 0xFF4D,
    VBK = 0xFF4F,
    HDMA1 = 0xFF51,
    HDMA2 = 0xFF52,
    HDMA3 = 0xFF53,
    HDMA4 = 0xFF54,
    HDMA5 = 0xFF55,
    RP = 0xFF56,
    BCPS = 0xFF68,
    BCPD = 0xFF69,
    OCPS = 0xFF6A,
    OCPD = 0xFF6B,
    OPRI = 0xFF6C,
    SVBK = 0xFF70,
    PCM12 = 0xFF76,
    PCM34 = 0xFF77,
}