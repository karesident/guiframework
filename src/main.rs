#![no_std]
#![no_main]
#![feature(collections)]
#![feature(asm)]

#[macro_use]
extern crate collections;

extern crate stm32f7_discovery as stm32f7;

#[macro_use]
mod semi_hosting;

// Initialization routines for .data and .bss.
extern crate r0;
use stm32f7::{system_clock, board, embedded, sdram, lcd, i2c, touch};
use embedded::interfaces::gpio::{self, Gpio};

mod forms;
mod draw;
mod util;
mod area_container;

use util::sizes::BoundingBox;
use collections::Vec;
use forms::button::Button;

use area_container::AddForm;
use area_container::DrawArea;
use collections::boxed::Box;
use forms::form::Form;


fn main(hw: board::Hardware) -> ! {
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        ..
    } = hw;

    let mut gpio = Gpio::new(gpio_a,
                             gpio_b,
                             gpio_c,
                             gpio_d,
                             gpio_e,
                             gpio_f,
                             gpio_g,
                             gpio_h,
                             gpio_i,
                             gpio_j,
                             gpio_k);

    system_clock::init(rcc, pwr, flash);

    // Enable all gpio ports.
    rcc.ahb1enr
        .update(|r| {
            r.set_gpioaen(true);
            r.set_gpioben(true);
            r.set_gpiocen(true);
            r.set_gpioden(true);
            r.set_gpioeen(true);
            r.set_gpiofen(true);
            r.set_gpiogen(true);
            r.set_gpiohen(true);
            r.set_gpioien(true);
            r.set_gpiojen(true);
            r.set_gpioken(true);
        });

    let led_pin = (gpio::Port::PortI, gpio::Pin::Pin1);
    let mut led = gpio.to_output(led_pin,
                                 gpio::OutputType::PushPull,
                                 gpio::OutputSpeed::Low,
                                 gpio::Resistor::NoPull)
        .expect("led pin already in use");

    led.set(true);

    // Initialize display.
    sdram::init(rcc, fmc, &mut gpio);
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    lcd.clear_screen();

    let button = forms::button::Button::new(util::sizes::BoundingBox {
                                                x: 2,
                                                y: 2,
                                                width: 10,
                                                height: 10,
                                            });

    // Initialize touch on display.
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    touch::check_family_id(&mut i2c_3).unwrap();

    //let color: lcd::Color = lcd::Color::from_hex(0xFF0000);
    //draw::draw_rectangle(30, 30, 100, 100, draw::convert_color_to_u16(color));

    let items = Vec::new();
    let bb = BoundingBox{x:15, y:15, width:10, height:10};
    println!("{}", bb.x);
    let button = Button::new(bb);
    println!("{}", button.get_clickable());

    let mut flow_container = area_container::FlowLayout{bounding_box:BoundingBox{x:10, y:10, width:30, height:30}, elements:items};
    let b = Box::new(button);
    flow_container.add_form(b);
    flow_container.draw_area();

    //let color: lcd::Color = lcd::Color::from_hex(0xFFFFFF);
    //draw::fill_rectangle(30, 30, 200, 200, draw::convert_color_to_u16(color));

    let mut last_led_toggle = system_clock::ticks();
    loop {
        let ticks = system_clock::ticks();

        if ticks - last_led_toggle >= 500 {
            let led_current = led.get();
            led.set(!led_current);
            last_led_toggle = ticks;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    //(copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    main(board::hw());
}
