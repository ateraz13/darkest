use cgmath::Basis;
// NOTE: Each Entity reference is bound to an lifecycle and everything that accesses
// and is only valid while the engine is actively in that life cycle.
// Once a new life cycle is started every reference to an entity needs to be renewed.
//
// NOTE: Destroying objects constantly could potentially be problematic.
// Using entity ID's needs to be properly though out because if an entity ID is held
// longer than the entity is alive it essencially acts as an dangling pointer.
//
// To be honest an enitity really has to only be held through out the duration of each frame.
// Each frame the entity ID can be looked up.
// Becuase entity cannot be destroyed during a frame it can be safely referenced by
// other entities because next frame after the entitty is destroyed all references
// need to be refreshed and the destruction of that entity can be communicated effectively
// to any other entity holding a refrence to another enitity on which notice the entity can
// decide to do what ever in needs to in that instance.
//
// To define how the system should work it is probably a best idea to start with defining
// what an entity may do and why would other entities want to ever reference other enitites
// and whether or not we can implement some for of indirection to avoid unnecessarry references.
//
//
// 1. Entities may be visually representable and thus the rendering system cares about relevant
// information which define how and when the entity is rendered.
//
// 2. Entities may be transformed in space and may require to access the coordinates other other
// entities to implement their transformational logic.
//
// 3. Entities may poses physical properties, this means they can interact with each other through
// the physics system. The information can be potentially passed around by the physics system
// without potentially requiring to pass around concrete entity references.
//
// 4. Entities may poses synchoronised state and be logically dependend on other entities, think
// about an AI system where NPC's interact with each other. However if the game logic is implemented
// from the top down it can be avoided. "Top down" means the game logic is handled by code that
// watches over entities and acts as a manager to for them. Instead of entities interacting with
// each other as if their making decisions, an overarching code can instead manipulate the entities
// to simulate as if the entities interacted. My description may be little confusing because I
// need to think about this approach a litle more but this should as a reminder for me.
// Essentially instead of doing somethign like this
// for entity in entities {
//       entity.think(); // All the logic lays within think()
// }
//
// We can do something like this.
//
//
// let decision_graph = {NpmGroup::Community::make_relation_graph(&entity_group)};
//
// for npc in &mut entity_group {
//      let state = decision_graph[npc.id];
//      // implement corresponding logic here
// }
//
// Essentially avoid making everything overly generic just write the damn code, this is not Unreal
// or Unity and that's it.
//
// etc.
//
// Entity definitions can be very specific to the game and they don't have to be generic.
// With new game come new entity definitions and that's the work flow instead of encoding
// every possible things a game might do just implement a thing the game needs and let
// other games implement their own thing.
//
//
// Entity storage must be organised in a fashion that benefits linear accesses by subsystems
// that occupy most of the frame time. For example if the rendering system takes the most
// time and iteracts and deals with entities most frequently than the entity storage should be
// optimised for that use case. In another case where the physics system does more work and
// and takes up more computational resources that the entity store should be optimized in such a
// way to benefit the physics system.
//
// Making the system extensively versatile is the root of all evil, it unnecesarrily complicated
// things and it leads to useless thinking and abstractions. Editing and writing code that
// is needlesly non specific is often very painful because most of the time is usually spent
// messing around trying to come up with some smart abstraction that is capable of encapsulating
// every possible scenario and use case which is extremely impractical in most situation.
// Sometimes it is very useful to create abstractions that eliviate some complexities but
// once complexity is no longer the problem and rather lazyness more often than not it
// building highly generic abstractions becomes a burden than an aid.
//
// Data transformation lays at the heart of computing and that's where decisions should stem from.
// Computer is a machine and so treat it like one!
//
// Game should be programmed with some hardware specification in mind and then everyhing in
// the code should be layed around in respect it. More powerful hardware should provide
// featues to improve performance where possible and less powerful hardware can be supported
// separately if desirable.
//



struct EntitiyRef {
    lifecycle: u16;
    id: u16;
}

struct EntityStore {

}

struct World {

}
