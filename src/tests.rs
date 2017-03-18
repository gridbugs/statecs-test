use generated::generated_0_a;
use generated::generated_0_b;

use generated::generated_0_a::Ecs as EcsA;
use generated::generated_0_b::Ecs as EcsB;

use generated::generated_0_a::Entity as EntityA;
use generated::generated_0_a::EntityMut as EntityMutA;

#[test]
fn component_set_single_bitfield() {
    let mut set = generated_0_a::ComponentSet::new();

    assert!(set.is_empty());

    set.insert_a();

    assert!(!set.is_empty());
    assert!(set.contains_a());
    assert!(!set.contains_b());

    set.insert_b();
    assert!(set.contains_b());

    set.remove_a();
    assert!(!set.contains_a());
    assert!(set.contains_b());

    set.clear();
    assert!(!set.contains_b());
    assert!(set.is_empty());
}

#[test]
fn component_set_multi_bitfield() {
    let mut set = generated_0_b::ComponentSet::new();

    assert!(set.is_empty());

    set.insert_a();

    assert!(!set.is_empty());
    assert!(set.contains_a());
    assert!(!set.contains_b());

    set.insert_b();
    assert!(set.contains_b());

    set.remove_a();
    assert!(!set.contains_a());
    assert!(set.contains_b());

    set.clear();
    assert!(!set.contains_b());
    assert!(set.is_empty());

}

#[test]
fn set_iter() {

    let mut ecs_a = generated_0_a::EcsCtx::new();
    let mut ecs_b = generated_0_b::EcsCtx::new();

    ecs_a.insert_solid(2);
    ecs_a.insert_solid(4);
    ecs_a.insert_solid(6);
    ecs_a.insert_collider(3);
    ecs_a.insert_opaque(5);


    ecs_b.insert_solid(2);
    ecs_b.insert_solid(4);
    ecs_b.insert_solid(6);
    ecs_b.insert_collider(3);
    ecs_b.insert_opaque(5);

    let mut solid_iter_a = ecs_a.id_iter_solid();
    let mut solid_iter_b = ecs_b.id_iter_solid();

    assert_eq!(solid_iter_a.next(), solid_iter_b.next());
    assert_eq!(solid_iter_a.next(), solid_iter_b.next());
    assert_eq!(solid_iter_a.next(), solid_iter_b.next());
}

#[test]
fn entity_ref() {

    let mut ecs_a = generated_0_a::EcsCtx::new();

    ecs_a.insert_a(0, 4);

    let entity = ecs_a.entity(0);

    assert_eq!(entity.copy_a(), Some(4));
}

#[test]
fn action() {

    let mut ecs = generated_0_a::EcsCtx::new();
    let mut action = generated_0_a::EcsAction::new();

    {
        let mut entity = action.entity_mut(0);
        entity.insert_solid();
        entity.insert_a(100);
    }

    ecs.commit(&mut action);

    {
        let entity = ecs.entity(0);
        assert!(entity.solid());
        assert_eq!(entity.copy_a(), Some(100));
    }

    action.delete_solid(0);
    ecs.commit(&mut action);

    {
        let entity = ecs.entity(0);
        assert!(!entity.solid());
    }

    action.insert_a(1, 200);
    action.insert_a(2, 300);

    ecs.commit(&mut action);

    assert_eq!(ecs.get_copy_a(0), Some(100));
    assert_eq!(ecs.get_copy_a(1), Some(200));
    assert_eq!(ecs.get_copy_a(2), Some(300));

    action.swap_a(0, 1);
    action.swap_a(1, 2);

    ecs.commit(&mut action);


    assert_eq!(ecs.get_copy_a(0), Some(200));
    assert_eq!(ecs.get_copy_a(1), Some(300));
    assert_eq!(ecs.get_copy_a(2), Some(100));
}

#[test]
fn action_iterator() {

    let mut ecs = generated_0_a::EcsCtx::new();
    ecs.insert_a(0, 100);
    ecs.insert_a(1, 200);
    ecs.insert_a(2, 300);
    ecs.insert_a(3, 400);
    ecs.insert_a(4, 400);

    let mut action = generated_0_a::EcsAction::new();
    action.insert_a(0, 500);
    action.swap_a(1, 2);
    action.swap_a(3, 5);
    action.delete_a(4);

    let mut pos = action.positive_copy_iter_a(&ecs);
    let mut neg = action.negative_iter_a(&ecs);

    assert_eq!(pos.next(), Some((0, 500)));
    assert_eq!(pos.next(), Some((1, 300)));
    assert_eq!(pos.next(), Some((2, 200)));
    assert_eq!(pos.next(), Some((5, 400)));
    assert_eq!(pos.next(), None);

    assert_eq!(neg.next(), Some(4));
    assert_eq!(neg.next(), Some(3));
    assert_eq!(neg.next(), None);
}
