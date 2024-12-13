//! Run this file with `cargo test --test 03_state_transition_enum`.

// The #[derive] attribute enables nicer error messages in tests.
// It will be explained in later weeks.
#[derive(Debug, PartialEq, Eq)]
enum ComputerState {
    Off,
    Running {
        /// Time from the start of the computer.
        uptime: u32,
        /// Time since the last mouse move (or since start if no mouse move has happened).
        idle_time: u32,
    },
    Sleeping {
        /// Time from the start of the computer.
        uptime: u32,
        /// Time since the last transition to sleep.
        sleep_time: u32,
    },
}

enum Event {
    TurnOn,
    TurnOff,
    PassTime(u32),
    MoveMouse,
}

// TODO: Implement the `pc_transition` function.
// A computer can be in three states (off, running or sleeping).
// It can receive four events (turn on, turn off, pass some amount of time and mouse move).
//
// When the PC is running or sleeping, it remembers the time since it was started (`uptime`).
// When the PC is running, it also remembers `idle_time` (time since last mouse move).
// When the PC is sleeping, it also remembers `sleep_time` (time since going to sleep).
//
// Here are the rules that the computer should abide by:
// 1) When `TurnOn` happens, if the PC is off, it switches to `Running`. Otherwise nothing happens.
// 2) When `TurnOff` happens, the PC switches to `Off`.
// 3) When `MoveMouse` happens:
//   - if the PC is sleeping, the PC switches to `Running`.
//   - if the PC is running, it resets its `idle_time` to zero.
// 4) When `PassTime(time)` happens, and the PC is on, it increments its `uptime` by `time`. Then:
//   - If the PC is running and its `idle_time` is larger than 1000, it switches to `Sleeping`.
//   - If the PC is sleeping and its `sleep_time` is larger than 500, it switches to `Off`.
//
// Try to avoid a fallthrough case (`_ => ...`) and explicitly enumerate all
// variants. Use "or patterns" (A | B | C) to group variants together.
// It should be possible to express the described logic with 9 (or less) match arms.
//
// Compare this approach with using a struct (which we did on the lesson).
// Is it easier with an enum or with a struct?

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{pc_transition, ComputerState, Event};

    #[test]
    fn turn_off_when_off() {
        assert_eq!(
            pc_transition(ComputerState::Off, Event::TurnOff),
            ComputerState::Off
        );
    }

    #[test]
    fn turn_off_when_running() {
        assert_eq!(
            pc_transition(
                ComputerState::Running {
                    uptime: 34,
                    idle_time: 43
                },
                Event::TurnOff
            ),
            ComputerState::Off
        );
    }

    #[test]
    fn turn_off_when_sleeping() {
        assert_eq!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 34,
                    sleep_time: 43
                },
                Event::TurnOff
            ),
            ComputerState::Off
        );
    }

    #[test]
    fn turn_on_when_off() {
        assert_eq!(
            pc_transition(ComputerState::Off, Event::TurnOn),
            ComputerState::Running {
                uptime: 0,
                idle_time: 0
            }
        );
    }

    #[test]
    fn turn_on_when_running() {
        assert_eq!(
            pc_transition(
                ComputerState::Running {
                    uptime: 1,
                    idle_time: 2
                },
                Event::TurnOn
            ),
            ComputerState::Running {
                uptime: 1,
                idle_time: 2
            }
        );
    }

    #[test]
    fn turn_on_when_sleeping() {
        assert_eq!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 3,
                    sleep_time: 4
                },
                Event::TurnOn
            ),
            ComputerState::Sleeping {
                uptime: 3,
                sleep_time: 4
            }
        );
    }

    #[test]
    fn pass_time_off() {
        assert_eq!(
            pc_transition(ComputerState::Off, Event::PassTime(10)),
            ComputerState::Off
        );
    }

    #[test]
    fn pass_time_running() {
        assert_eq!(
            pc_transition(
                ComputerState::Running {
                    uptime: 123,
                    idle_time: 10
                },
                Event::PassTime(14)
            ),
            ComputerState::Running {
                uptime: 137,
                idle_time: 24
            }
        );
    }

    #[test]
    fn pass_time_go_to_sleep() {
        assert_eq!(
            pc_transition(
                ComputerState::Running {
                    uptime: 800,
                    idle_time: 900
                },
                Event::PassTime(120)
            ),
            ComputerState::Sleeping {
                uptime: 920,
                sleep_time: 20
            }
        );
    }

    #[test]
    fn pass_time_shutdown() {
        assert_eq!(
            pc_transition(
                ComputerState::Running {
                    uptime: 800,
                    idle_time: 100
                },
                Event::PassTime(10000)
            ),
            ComputerState::Off
        );
    }

    #[test]
    fn pass_time_sleeping() {
        assert_eq!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 800,
                    sleep_time: 100
                },
                Event::PassTime(120)
            ),
            ComputerState::Sleeping {
                uptime: 920,
                sleep_time: 220
            }
        );
    }

    #[test]
    fn pass_time_sleeping_turn_off() {
        assert_eq!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 640,
                    sleep_time: 450
                },
                Event::PassTime(60)
            ),
            ComputerState::Off
        );
    }

    #[test]
    fn mouse_move_off() {
        assert_eq!(
            pc_transition(ComputerState::Off, Event::MoveMouse),
            ComputerState::Off
        );
    }

    #[test]
    fn mouse_move_running() {
        assert_eq!(
            pc_transition(
                ComputerState::Running {
                    uptime: 100,
                    idle_time: 100
                },
                Event::MoveMouse
            ),
            ComputerState::Running {
                uptime: 100,
                idle_time: 0
            }
        );
    }

    #[test]
    fn mouse_move_wake() {
        assert_eq!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 500,
                    sleep_time: 40
                },
                Event::MoveMouse
            ),
            ComputerState::Running {
                uptime: 500,
                idle_time: 0
            }
        );
    }

    #[test]
    fn complex_transition_1() {
        let mut pc = ComputerState::Off;
        let events = [
            Event::TurnOn,
            Event::PassTime(100),
            Event::PassTime(50),
            Event::MoveMouse,
            Event::PassTime(500),
            Event::PassTime(600),
            Event::PassTime(100),
            Event::MoveMouse,
            Event::PassTime(20),
            Event::PassTime(100),
        ];
        for event in events {
            pc = pc_transition(pc, event);
        }
        assert_eq!(
            pc,
            ComputerState::Running {
                uptime: 1470,
                idle_time: 120
            }
        );
    }

    #[test]
    fn complex_transition_2() {
        let mut pc = ComputerState::Off;
        let events = [
            Event::TurnOn,
            Event::PassTime(100),
            Event::PassTime(50),
            Event::MoveMouse,
            Event::PassTime(500),
            Event::PassTime(600),
            Event::TurnOff,
            Event::MoveMouse,
            Event::PassTime(600),
            Event::TurnOn,
            Event::PassTime(100),
            Event::MoveMouse,
            Event::PassTime(20),
            Event::PassTime(100),
            Event::PassTime(1000),
            Event::TurnOn,
            Event::PassTime(150),
        ];
        for event in events {
            pc = pc_transition(pc, event);
        }
        assert_eq!(
            pc,
            ComputerState::Sleeping {
                uptime: 1370,
                sleep_time: 270
            }
        );
    }
}
