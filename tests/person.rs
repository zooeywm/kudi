#[cfg(test)]
mod tests {
    #![allow(clippy::explicit_auto_deref)]
    #![allow(clippy::borrow_deref_ref)]
    #![allow(unused_mut)]

    use std::{pin::Pin, rc::Rc, sync::Arc};

    use kudi_macros::DepInj;

    #[derive(DepInj, Clone)]
    #[target(Person)]
    struct PersonState {
        name: String,
    }

    #[derive(Clone)]
    struct TestContainer {
        person: PersonState,
    }

    impl AsRef<PersonState> for TestContainer {
        fn as_ref(&self) -> &PersonState {
            &self.person
        }
    }

    impl AsMut<PersonState> for TestContainer {
        fn as_mut(&mut self) -> &mut PersonState {
            &mut self.person
        }
    }

    impl From<TestContainer> for PersonState {
        fn from(val: TestContainer) -> Self {
            val.person
        }
    }

    #[test]
    fn test_person_generated_methods() {
        let person = PersonState {
            name: "Alice".into(),
        };
        let mut container = TestContainer {
            person: person.clone(),
        };
        let person_inj: Person<TestContainer> = Person::inj(container.clone());
        // Test basic inj/prj methods
        let _: TestContainer = person_inj.prj();

        // Test reference methods
        let person_inj_ref: &Person<TestContainer> = Person::inj_ref(&container);
        let _: &TestContainer = person_inj_ref.prj_ref();

        // Test mutable reference methods
        let person_inj_ref_mut: &mut Person<TestContainer> = Person::inj_ref_mut(&mut container);
        let _: &mut TestContainer = person_inj_ref_mut.prj_ref_mut();

        // Test Box methods
        let person_inj_box: Box<Person<TestContainer>> = Person::inj_box(Box::new(container.clone()));
        let _: Box<TestContainer> = person_inj_box.prj_box();

        // Test Rc methods
        let person_inj_rc: Rc<Person<TestContainer>> = Person::inj_rc(Rc::new(container.clone()));
        let _: Rc<TestContainer> = person_inj_rc.prj_rc();

        // Test Arc methods
        let person_inj_arc: Arc<Person<TestContainer>> = Person::inj_arc(Arc::new(container.clone()));
        let _: Arc<TestContainer> = person_inj_arc.prj_arc();

        // Test Pin reference methods
        let person_inj_pin_ref: Pin<&Person<TestContainer>> = Person::inj_pin_ref(Pin::new(&container));
        let _: Pin<&TestContainer> = person_inj_pin_ref.prj_pin_ref();

        // Test Pin mutable reference methods
        let mut person_inj_pin_ref_mut: Pin<&mut Person<TestContainer>> =
            Person::inj_pin_ref_mut(Pin::new(&mut container));
        let _: Pin<&mut TestContainer> = person_inj_pin_ref_mut.prj_pin_ref_mut();

        // Test Pin Box methods
        let person_inj_pin_box: Pin<Box<Person<TestContainer>>> =
            Person::inj_pin_box(Box::pin(container.clone()));
        let _: Pin<Box<TestContainer>> = person_inj_pin_box.prj_pin_box();

        // Test Pin Rc methods
        let person_inj_pin_rc: Pin<Rc<Person<TestContainer>>> =
            Person::inj_pin_rc(Rc::pin(container.clone()));
        let _: Pin<Rc<TestContainer>> = person_inj_pin_rc.prj_pin_rc();

        // Test Pin Arc methods
        let person_inj_pin_arc: Pin<Arc<Person<TestContainer>>> =
            Person::inj_pin_arc(Arc::pin(container.clone()));
        let _: Pin<Arc<TestContainer>> = person_inj_pin_arc.prj_pin_arc();

        // Test Deref trait
        assert_eq!(person.name, Person::inj_ref(&container).name);

        // Test DerefMut trait
        assert_eq!(person.name, Person::inj_ref_mut(&mut container).name);

        // Test From trait conversion
        let person_inj: Person<TestContainer> = Person::inj(container.clone());
        let _: PersonState = PersonState::from(person_inj);
    }
}
