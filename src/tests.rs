use generated::generated_0_a;
use generated::generated_0_b;

use generated::generated_0_a::Ecs as EcsA;
use generated::generated_0_b::Ecs as EcsB;
use generated::generated_0_a::Entity as EntityA;

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
