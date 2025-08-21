mod memory;
mod opcode;
mod registers;
mod timers;

use std::{fs::File, sync::Arc};

use crate::chip9::{
    display::Display,
    Keyboard,
};

use crate::errors::Chip9Error;
use memory::Memory;
use opcode::{Addr, Nib, OpCode};
use registers::Registers;
use timers::{Timer, TimerClock};

pub const PROGRAM_START: u16 = 0x200;
const STACK_DEPTH: usize = 16;
const SPRITE_SIZE: u16 = 5;

pub struct CPU {
    // Registers
    regs: Registers, // 16 general purpose 8-bit registers
    idx: Addr, // 12-bit address register
    dt: Arc<Timer>, // delay timer
    st: Arc<Timer>, // sound timer
    pc: Addr, // Program counter
    sp: u8, // Stack pointer
    stack: [Addr; STACK_DEPTH], // 16 12-bit stack fields
    mem: Memory,

    _timer_clock: TimerClock
}

impl CPU {
    pub fn new() -> Self {
        let regs = Registers::new();
        let idx = Addr::new();
        let dt = Arc::new(Timer::new());
        let st = Arc::new(Timer::new());
        let pc = Addr::from(PROGRAM_START);
        let sp = 0x00;
        let stack = [Addr::new(); STACK_DEPTH];

        let mem = Memory::new();

        let mut _timer_clock = TimerClock::new();
        _timer_clock.register(dt.clone());
        _timer_clock.register(st.clone());
        _timer_clock.start();

        Self {
            regs,
            idx,
            dt,
            st,
            pc,
            sp,
            stack,
            mem,
            _timer_clock
        }
    }

    pub fn load_program(&mut self, file: &File) -> Result<(), Box<dyn std::error::Error>> {
        self.mem.load_from_file(file)
    }

    pub fn shutdown(&mut self) { // should be called only on started
        self._timer_clock.shutdown();
    }

    fn fetch(&mut self) -> Result<OpCode, Chip9Error> {
        let instruction = self.mem.get_instruction(self.pc);
        self.pc += 2;

        OpCode::decode(instruction)
    }

    pub fn execute(&mut self, display: &mut Display, keyboard: &mut Keyboard) -> Result<(), Chip9Error> {
        let opcode = self.fetch()?;

        match opcode {
            OpCode::NoOp => (),
            OpCode::ClearScreen => self.cleared_screen(display),
            OpCode::Return => self.return_subroutine(),
            OpCode::Jump(addr) => self.jump_addr(addr),
            OpCode::Call(addr) => self.call_addr(addr),
            OpCode::SkipEqualByte(x, byte) => self.skip_eq_byte(x, byte),
            OpCode::SkipNotEqualByte(x, byte) => self.skip_neq_byte(x, byte),
            OpCode::SkipEqualReg(x, y) => self.skip_eq_reg(x, y),
            OpCode::LoadByte(x, byte) => self.load_byte(x, byte),
            OpCode::AddByte(x, byte) => self.add_byte(x, byte),
            OpCode::LoadReg(x, y) => self.load_reg(x, y),
            OpCode::OrReg(x, y) => self.or_reg(x, y),
            OpCode::AndReg(x, y) => self.and_reg(x, y),
            OpCode::XorReg(x, y) => self.xor_reg(x, y),
            OpCode::AddReg(x, y) => self.add_reg(x, y),
            OpCode::SubReg(x, y) => self.sub_reg(x, y),
            OpCode::ShiftRight(x, _) => self.shr_reg(x),
            OpCode::SubNot(x, y) => self.subn_reg(x, y),
            OpCode::ShiftLeft(x, _) => self.shl_reg(x),
            OpCode::SkipNotEqualReg(x, y) => self.skip_neq_reg(x, y),
            OpCode::LoadIndex(addr) => self.load_idx(addr),
            OpCode::JumpV0(addr) => self.jump_v0(addr),
            OpCode::RandomByte(x, byte) => self.random_byte(x, byte),
            OpCode::Draw(x, y, n) => self.draw(x, y, n, display),
            OpCode::SkipKeyPressed(x) => self.skip_key_pressed(x, keyboard),
            OpCode::SkipKeyNotPressed(x) => self.skip_key_not_pressed(x, keyboard),
            OpCode::LoadDelay(x) => self.load_delay(x),
            OpCode::WaitKey(x) => self.wait_key(x, keyboard),
            OpCode::SetDelay(x) => self.set_delay(x),
            OpCode::SetSound(x) => self.set_sound(x),
            OpCode::AddToIndex(x) => self.add_idx(x),
            OpCode::LoadFont(x) => self.load_sprite(x),
            OpCode::LoadBCD(x) => self.load_bcd(x),
            OpCode::StoreRegs(x) => self.store_regs(x),
            OpCode::LoadRegs(x) => self.load_regs(x),
        }

        Ok(())
    }

    pub fn sound_timer(&self) -> u8 {
        self.st.get()
    }

    fn return_subroutine(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn cleared_screen(&mut self, display: &mut Display) {
        display.clear();
    }

    fn jump_addr(&mut self, addr: Addr) {
        self.pc = addr;
    }

    fn call_addr(&mut self, addr: Addr) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = addr;
    }

    fn skip_eq_byte(&mut self, vx: Nib, byte: u8) {
        if self.regs[vx] == byte {
            self.pc += 2;
        }
    }

    fn skip_neq_byte(&mut self, vx: Nib, byte: u8) {
        if self.regs[vx] != byte {
            self.pc += 2;
        }
    }

    fn skip_eq_reg(&mut self, vx: Nib, vy: Nib) {
        if self.regs[vx] == self.regs[vy] {
            self.pc += 2;
        }
    }

    fn load_byte(&mut self, vx: Nib, byte: u8) {
        self.regs[vx] = byte;
    }

    fn add_byte(&mut self, vx: Nib, byte: u8) {
        self.regs[vx] = self.regs[vx].wrapping_add(byte);
    }

    fn load_reg(&mut self, vx: Nib, vy: Nib) {
        self.regs[vx] = self.regs[vy];
    }

    fn or_reg(&mut self, vx: Nib, vy: Nib) {
        self.regs[vx] |= self.regs[vy];
    }

    fn and_reg(&mut self, vx: Nib, vy: Nib) {
        self.regs[vx] &= self.regs[vy];
    }

    fn xor_reg(&mut self, vx: Nib, vy: Nib) {
        self.regs[vx] ^= self.regs[vy];
    }

    fn add_reg(&mut self, vx: Nib, vy: Nib) {
        let (sum, carry) = self.regs[vx].overflowing_add(self.regs[vy]);
        self.regs.set_flag(carry as u8);
        self.regs[vx] = sum;
    }

    fn sub_reg(&mut self, vx: Nib, vy: Nib) {
        let (diff, borrow) = self.regs[vx].overflowing_sub(self.regs[vy]);
        self.regs.set_flag((!borrow) as u8);
        self.regs[vx] = diff;
    }

    fn shr_reg(&mut self, vx: Nib) {
        let underflow = self.regs[vx] & 1;
        self.regs.set_flag(underflow);
        self.regs[vx] >>= 1;
    }

    fn subn_reg(&mut self, vx: Nib, vy: Nib) {
        let (diff, borrow) = self.regs[vy].overflowing_sub(self.regs[vx]);
        self.regs.set_flag((!borrow) as u8);
        self.regs[vx] = diff;
}

    fn shl_reg(&mut self, vx: Nib) {
        let overflow = self.regs[vx] >> 7;
        self.regs.set_flag(overflow);
        self.regs[vx] <<= 1;
    }

    fn skip_neq_reg(&mut self, vx: Nib, vy: Nib) {
        if self.regs[vx] != self.regs[vy] {
            self.pc += 2;
        }
    }

    fn load_idx(&mut self, addr: Addr) {
        self.idx = addr;
    }

    fn jump_v0(&mut self, addr: Addr) {
        self.pc = addr + self.regs.v0().into();
    }

    fn random_byte(&mut self, vx: Nib, byte: u8) {
        let rnd: u8 = rand::random();
        self.regs[vx] = byte & rnd;
    }

    fn draw(&mut self, vx: Nib, vy: Nib, height: Nib, display: &mut Display) {
        // Read sprite from memory
        let sprite = (0..height.value())
            .map(|offset| self.mem.read_byte(self.idx + offset as u16));

        let x = self.regs[vx] as usize;
        let y = self.regs[vy] as usize;

        // Draw sprite and set collision flag
        let collision = display.draw(x, y, sprite);
        self.regs.set_flag(collision as u8);
    }

    // Ennn - Keyboard operations
    fn skip_key_pressed(&mut self, vx: Nib, keyboard: &mut Keyboard) {
        if keyboard.is_key_pressed(self.regs[vx]) {
            self.pc += 2;
        }
    }

    fn skip_key_not_pressed(&mut self, vx: Nib, keyboard: &mut Keyboard) {
        if !keyboard.is_key_pressed(self.regs[vx]) {
            self.pc += 2;
        }
    }

    fn load_delay(&mut self, vx: Nib) {
        self.regs[vx] = self.dt.get();
    }

    fn wait_key(&mut self, vx: Nib, keyboard: &mut Keyboard) {
        if let Some(key) = keyboard.get_key_press() {
            self.regs[vx] = key;
        } else {
            self.pc -= 2;
        }
    }

    fn set_delay(&mut self, vx: Nib) {
        self.dt.load(self.regs[vx]);
    }

    fn set_sound(&mut self, vx: Nib) {
        self.st.load(self.regs[vx]);
    }

    fn add_idx(&mut self, vx: Nib) {
        self.idx += self.regs[vx] as u16;
    }

    fn load_sprite(&mut self, vx: Nib) {
        self.idx = Addr::from( SPRITE_SIZE * self.regs[vx] as u16);
    }

    fn load_bcd(&mut self, vx: Nib) {
        self.mem.write_byte(self.idx, self.regs[vx] / 100);
        self.mem.write_byte(self.idx + 1, (self.regs[vx] % 100) / 10);
        self.mem.write_byte(self.idx + 2, self.regs[vx] % 10);
    }

    fn store_regs(&mut self, vx: Nib) {
        for i in 0..=vx.value() {
            let nib = Nib::from(i);
            self.mem.write_byte(self.idx + i as u16, self.regs[nib]);
        }
        self.idx += vx.value() as u16 + 1;
    }

    fn load_regs(&mut self, vx: Nib) {
        for i in 0..=vx.value() {
            let nib = Nib::from(i);
            self.regs[nib] = self.mem.read_byte(self.idx + i as u16);
        }
        self.idx += vx.value() as u16 + 1;
    }
}