struct User {
    id: u32,
}

enum MaybeAdmin {
    Admin(Admin),
    Denied,
}

fn as_admin(user: User) -> MaybeAdmin {
    MaybeAdmin::Admin(Admin(user))
}

struct Admin(User);

fn is_admin(user: &User) -> bool {
    user.id == 0
}

fn show_dashboard() {}
fn show_database(admin: Admin) {}

fn admin_dashboard(user: User) -> u32 {
    if !is_admin(&user) {
        return 400;
    }
    show_dashboard();
    200
}

fn admin_database(user: User) -> u32 {
    match as_admin(user) {
        MaybeAdmin::Admin(admin) => {
            show_database(admin);
            200
        }
        MaybeAdmin::Denied => 400,
    }
}

fn main() {}
