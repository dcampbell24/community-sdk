//! # Example - AverageBandPowers
//! The average band power for a specific channel from the latest epoch with
//! 0.5 seconds step size and 2 seconds window size
//! This example is used for single connection.

extern crate emotiv_community_sdk;

use std::ffi::CString;

use emotiv_community_sdk::edk_sys::{self, IEE_Event_enum, IEE_DataChannels_enum};

static CHANNELS: [IEE_DataChannels_enum; 14] = [
    IEE_DataChannels_enum::IED_AF3,
    IEE_DataChannels_enum::IED_F7,
    IEE_DataChannels_enum::IED_F3,
    IEE_DataChannels_enum::IED_FC5,
    IEE_DataChannels_enum::IED_T7,
    IEE_DataChannels_enum::IED_P7,
    IEE_DataChannels_enum::IED_Pz,
    IEE_DataChannels_enum::IED_O2,
    IEE_DataChannels_enum::IED_P8,
    IEE_DataChannels_enum::IED_T8,
    IEE_DataChannels_enum::IED_FC6,
    IEE_DataChannels_enum::IED_F4,
    IEE_DataChannels_enum::IED_F8,
    IEE_DataChannels_enum::IED_AF4,
];

fn main() {
    unsafe {
        let e_event = edk_sys::IEE_EmoEngineEventCreate();
        let e_state = edk_sys::IEE_EmoStateCreate();

        let user: *mut u32 = &mut 0;
        let mut ready = false;

        let alpha: *mut f64 = &mut 0.0;
        let low_beta: *mut f64 = &mut 0.0;
        let high_beta: *mut f64 = &mut 0.0;
        let gamma: *mut f64 = &mut 0.0;
        let theta: *mut f64 = &mut 0.0;

        let engine_string = CString::new("Emotiv Systems-5").unwrap();
        if edk_sys::IEE_EngineConnect(engine_string.as_ptr()) != edk_sys::EDK_OK as i32 {
            println!("Emotiv Engine start up failed.");
            return;
        }

        println!("Start receiving Data!");
        println!("Theta, Alpha, Low_beta, High_beta, Gamma");

        loop {
            let state = edk_sys::IEE_EngineGetNextEvent(e_event);

            if state == edk_sys::EDK_OK as i32  {
                let event_type = edk_sys::IEE_EmoEngineEventGetType(e_event);
                edk_sys::IEE_EmoEngineEventGetUserId(e_event, user);

                if event_type == IEE_Event_enum::IEE_UserAdded {
                    println!("User added");
                    ready = true;
                }
            } else if state != edk_sys::EDK_NO_EVENT as i32 {
                println!("Internal error in Emotiv Engine!");
                break;
            }

            if ready {
                for channel in &CHANNELS {
                    let result = edk_sys::IEE_GetAverageBandPowers(
                        *user, *channel, theta, alpha, low_beta, high_beta, gamma
                    );
                    if result == edk_sys::EDK_OK as i32 {
                        println!("{}, {}, {}, {}, {}",
                                 *theta, *alpha, *low_beta, *high_beta, *gamma);
                    }
                }
            }
        }

        edk_sys::IEE_EngineDisconnect();
        edk_sys::IEE_EmoStateFree(e_state);
        edk_sys::IEE_EmoEngineEventFree(e_event);
        println!("Disconnected!");
    }
}
