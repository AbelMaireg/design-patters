fn main() {
    println!("Hello, world!");
    let i = Concrete {};
    i.execute();
}

trait Template {
    fn execute(&self) {
        self.task_1();
        self.task_2();
        self.hook_1();
        self.hook_2();
        self.ops_1();
        self.ops_2();
    }

    fn task_1(&self) {
        println!("task_1")
    }

    fn task_2(&self) {
        println!("task_2")
    }

    fn hook_1(&self) {}
    fn hook_2(&self) {}

    fn ops_1(&self);
    fn ops_2(&self);
}

struct Concrete;

impl Template for Concrete {
    fn ops_1(&self) {
        println!("ops_1");
    }

    fn ops_2(&self) {
        println!("ops_2");
    }

    fn hook_1(&self) {
        println!("optional hook method")
    }
}
