# Overview of the Project

## Individuals

Each group member has to develop an individual contribution, which is:

-   Robot AI, meaning, some basic logic, that uses tools and/or interfaces to control a robot in the world following a Mission
-   UI Displayer, which displays what a robot is doing in the world.

## Working Groups

We are organized in groups of 4 people, with some exceptions of 3 people. Each group will be responsible of developing:

-   1 virtual robot, it interacts with a world, moving, and doing some basic tasks on the world.
-   1 tool (sensor or actuator).
-   1 world generator or another tool.

## WGL (Working Group Leader)

One of the 4 people in the working group, they are responsible for voting changes to the Specifications (they are the only ones that can vote). The list of WGL can be found in **notes** below.

## WGC (Working Group Coordinator)

Also known as scapegoat, is responsible of making the various meetings productive, drafting each new version of the Specifications and to submit the shared code to the repo. Our WGC is [Matteo Possamai](https://t.me/poss03251)

# SW Components

## Common Crate

The Commons Crate is the only crate that is shared among all groups. It contains following modules:

-   World
-   Interface
-   Runner
-   Energy
-   Test
-   Utils

## World

A World is a 2d grid whose cells contain various things: grass, water, fire, trees, bins. It is defined in the Commons Crate.

## Interfaces

The only way to interact with the World is through Interfaces, these are described in the doc and only implemented in the Commons Crate.

Interfaces allow limited sensing (reading) of the world and limited acting (writing) on the world. (except for the debug interface, **test purposes only**)

Some Interfaces require some Energy to work, and Energy is created in the Commons Crate. Also Interfaces are implemented in the Commons Crate.

## Runner

The module that manage the runtime logic and contains both the definition of a robot and the trait it must implement.

## Energy

The module that contains the definition of the energy and its methods.

## Test

Here you can find the tests for the Commons Crate, you can run them with the command `cargo test` in the Commons Crate folder or, using a JetBrains IDE, you can run them with `test --package robotics_lib --lib tests::testing -- --exact` as command.

## Utils

Here you can find all the utilities functions used in the Commons Crate.

## Robot

A robot receives a fixed amount of energy each tick, with which it should hopefully complete its task, using sensors and actuators (developed by other groups).

## Tools

In order for a Robot to use an Interface, teams will implement Tools.
These Tools combine Interfaces to grant the robot elaborate functionalities provided the Robot has enough energy.

For example, a Tool can put out all the fires in a row, by iterating over the put-out-fire and move Interfaces.

---

# Stages of Development

1. Early Stage: During the first few weeks, Working Group Leaders will meet in different meetings to agree on the specs and fix them. This corrispond to the branch out of sub-Working Groups.
2. Implementation of the Commons Crate: initial individual contributions and component development can begin: world generator and sensors; within each working group.
3. Freezing the Specification (**max 21st class**): As the semester approaches its end, the Specifications will be considered _frozen_ with no further modifications allowed, except for correcting typos. To unfreeze the Specifications, the WGC needs consensus from the WGL (&ge; 51%). If the request is deemed reasonable, the Specifications is unfrozen for one meeting, finalized, and then frozen again. Unfreezing is discouraged as it may lead to changes in **your** code, and excessive unfreezing reflects negatively on the WGL and WGC evaluations. It is advised to work towards a stable Specifications to avoid the need for unfreezing.
4. Faire (**5th Dec**): At some point in Povo, there will be an actual faire, where each group will sell and buy sensors and world generators. You have to **come with working versions of your sensors, show that they comply to the agreed Specification** and convince other groups to commit to your code and not to other groups. During the Faire, each group must commit to use: **at least one World Generator, at least 7 Sensors**. Each group's choices will be registered at the end of the faire. These choices are final, and in the final project evaluation, your robot will have to run with the crates you are committed to. (The first three groups whose code is committed-to the most will have a grade bonus)
5. Robot Development and Maintenance: During this phase, different working groups will be tasked with developing their own robot, with only the sensors purchased from other groups. In addition, each group will have to provide support for previously produced components to client groups.

---

# Class Code Repository

## Setup

Each group's sensors, plus any shared code will be hosted on the class registry, available at: https://advancedprogramming.disi.unitn.it/ .

You will recive your personal access token for pushing and pulling crates from the registry. The group will be registered with the access tokens of all its members in order for you to push and pull group code.

You need to add a file to your project in `.cargo/config.toml` with the following lines:

```
[registries]
kellnr = { index = "git://advancedprogramming.disi.unitn.it/index",
token = "put your token here between quotes"}
```

## Pulling code

You can then edit the file `Cargo.toml` and include lines such as these:

```
[dependencies]
test = { version = "0.1.1", registry = "kellnr"}
```

where instead of `test`, you import your colleagues component.

## Pushing your code

In order to push your market, you need to edit the file `Cargo.toml` and include lines such as these

```
[package]
name = " your robot name"
version = "0.1.2"
edition = "2023"
authors = ["your names"]
publish = ["kellnr"]
#do not modify the "publish" key, this is the name of the registry
# as read from the ".cargo/config.toml" file
```

and use the command
`cargo publish`
to publish the crate to the mentioned registry.

# Notes

## Code principles

In all the code you write (for the WG, or in the implementations of your group, or the final project) you must keep present this principles:

-   each of the crates your group publishes on the Registry must have the group name (or an acronym) in its own name. The names must be explicit of the crate content.
-   no unsafe code
-   no undocumented panic!
-   the code must be extensively tested, and you should not preclude crates downstream to test their code
-   expose through public interfaces only what is strictly needed, and only that
-   write idiomatic code

Please

## List of WGL

-   Rust and Furious: Sara Francavilla
-   Rusty Krab: [Loan Gabriel Duta](https://t.me/bagzziii)
-   Fe₂O₃: [Morgana Pasquini](https://t.me/Rulaanxxvii)
-   The rust of us: [Alessio Amiri](https://t.me/SpaghettiMan69)
-   Rustbeef: [Thomas Pasquali](https://t.me/thom_pasqui)
-   Cargo commandos: [Niccolò Eccel](https://t.me/Neo3010)
-   Crab Rave: [Alessio Faieta](https://t.me/alessiofaieta)
-   ＲｕｓｔｙＰｒｉｐｙａｔ ☭ ʖ ☭: [Toniolo Marco](https://t.me/marco_toniolo)
-   Ownersheeps: Gabriele Bazzanella Bauer
-   I Rustici: Federico Menegoz
-   Rust-eze: [Guglielmo Boi](https://t.me/guglielmoboi)
-   Another one bytes the Rust: [Corradini Dimitri](https://t.me/dimi56497)
-   tRust us: Matteo Frigo
-   WhoNeedsGV: Marco Antonio Murru
-   #![allow(bad_code)]: [Silvanus Bordignon](https://t.me/silvanusbordignon)
-   do not panic!(): Marco Pulze
-   tyrannosauRust-rex: [Giovanni Foletto](https://t.me/GiovanniFoletto)
-   (\\/)('-')(\\/): [Nicolò Marchini](https://t.me/praisethefab)
-   .unwrap().unwrap().unwrap(): [Andrea Bissoli](https://t.me/AndreaBissoli)
-   Pattern-Matching Pioneers: Davide José Paci
-   antiRust: Andrea Richichi
-   I fRustati: [Denise Comincioli](https://t.me/DeathOnABicycle)
-   Turbofish Team: [Gianluca Rigatti](https://t.me/gian03r)
-   Rust in peace: [Podavini Luca](https://t.me/lucapoda)
-   OhCrab: Kateřina Průšová
-   OxidizingAgents: [Filippo Lollato](https://t.me/lolfilippo)
-   Holy Crab: Davide Pedrotti
-   Ραστανιδουμεν:Artem Buev
-   Rustafariani: Rodrigo Salas
-   1v1 on Rust?: Claudio Vozza
-   Ghost Rusters: [Ludovico Cappellato](https://t.me/ludo_cappe)
-   arRusticini: [Federico Cucino](https://t.me/federicocuci)
-   CongRUSTulazioni: Marco Demo
