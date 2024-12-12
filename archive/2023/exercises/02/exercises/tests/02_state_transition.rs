//! Run this file with `cargo test --test 01_calc`.

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

// Implement the `pc_transition` function.
// A computer can be in three states (off, running or sleeping).
// It can receive four events (turn on, turn off, pass some amount of time and move mouse).
//
// When the PC is on (running or sleeping), it remembers the time since it was started (`uptime`).
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
// variants. But still try to use or patterns (A | B | C) to group variants together!
// It should be possible to express this logic in 9 match arms or less.
// After passing all tests, try to go through the match statement again and

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{pc_transition, ComputerState, Event};

    #[test]
    fn turn_off_when_off() {
        // The matches!(<variable>, <pattern>) macro returns `true` if <variable> matches the
        // given <pattern>.
        // We could have nicer error messages using `assert_eq!`, but for that we need to know traits
        // first :) Stay tuned.
        assert!(matches!(
            pc_transition(ComputerState::Off, Event::TurnOff),
            ComputerState::Off
        ));
    }

    #[test]
    fn turn_off_when_running() {
        assert!(matches!(
            pc_transition(
                ComputerState::Running {
                    uptime: 34,
                    idle_time: 43
                },
                Event::TurnOff
            ),
            ComputerState::Off
        ));
    }

    #[test]
    fn turn_off_when_sleeping() {
        assert!(matches!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 34,
                    sleep_time: 43
                },
                Event::TurnOff
            ),
            ComputerState::Off
        ));
    }

    #[test]
    fn turn_on_when_off() {
        assert!(matches!(
            pc_transition(ComputerState::Off, Event::TurnOn),
            ComputerState::Running {
                uptime: 0,
                idle_time: 0
            }
        ));
    }

    #[test]
    fn turn_on_when_running() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn turn_on_when_sleeping() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn pass_time_off() {
        assert!(matches!(
            pc_transition(ComputerState::Off, Event::PassTime(10)),
            ComputerState::Off
        ));
    }

    #[test]
    fn pass_time_running() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn pass_time_go_to_sleep() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn pass_time_sleeping() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn pass_time_sleeping_turn_off() {
        assert!(matches!(
            pc_transition(
                ComputerState::Sleeping {
                    uptime: 640,
                    sleep_time: 450
                },
                Event::PassTime(60)
            ),
            ComputerState::Off
        ));
    }

    #[test]
    fn mouse_move_off() {
        assert!(matches!(
            pc_transition(ComputerState::Off, Event::MoveMouse),
            ComputerState::Off
        ));
    }

    #[test]
    fn mouse_move_running() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn mouse_move_wake() {
        assert!(matches!(
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
        ));
    }

    #[test]
    fn complex_transition() {
        let pc = ComputerState::Off;
        let pc = pc_transition(pc, Event::TurnOn);
        let pc = pc_transition(pc, Event::PassTime(100));
        let pc = pc_transition(pc, Event::PassTime(50));
        let pc = pc_transition(pc, Event::MoveMouse);
        let pc = pc_transition(pc, Event::PassTime(500));
        let pc = pc_transition(pc, Event::PassTime(20));
        let pc = pc_transition(pc, Event::PassTime(600));
        let pc = pc_transition(pc, Event::PassTime(100));
        let pc = pc_transition(pc, Event::MoveMouse);
        let pc = pc_transition(pc, Event::PassTime(100));
        assert!(matches!(
            pc,
            ComputerState::Running {
                uptime: 1470,
                idle_time: 100
            }
        ));
    }
}
