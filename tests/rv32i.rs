pub fn run(
    emu: &mut Emulator,
    data: Vec<u8>,
    expected_xregs: &[u64; 32],
    expected_fregs: &[f64; 32],
) {
    let len = data.len() as u64;

    emu.is_debug = true;

    emu.initialize_dram(data);
    emu.initialize_pc(DRAM_BASE);

    emu.test_start(DRAM_BASE, DRAM_BASE + len);

    for (i, e) in expected_xregs.iter().enumerate() {
        assert_eq!(*e, emu.cpu.xregs.read(i as u64), "fails at {}", i);
    }
    for (i, e) in expected_fregs.iter().enumerate() {
        assert_eq!(
            (*e).to_bits(),
            emu.cpu.fregs.read(i as u64).to_bits(),
            "fails at {} expected {} but got {} ",
            i,
            *e,
            emu.cpu.fregs.read(i as u64)
        );
    }
}
#[test]
fn addi_rd_rs1_imm() {
    let mut emu = Emulator::new();

    let data = vec![
        0x93, 0x0F, 0x40, 0x00, // addi x31, x0,
    ];
}
