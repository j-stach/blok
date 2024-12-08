
use blok::*;

#[derive(Clone, Default)]
struct MyBlock {
    id: String
}

impl Block for MyBlock {
    type CreationInstructions = String;
    // NOTE:
    // Boilerplate for non-nodular block types: 
    // Connections are not included in this example.
    type ConnectionInstructions = ();

    fn create(id: &String) -> Self {
        MyBlock {
            id: id.to_owned(),
        }
    }

    fn void() -> Self {
        Self::default()
    }

    fn is_void(&self) -> bool {
        self.id.is_empty()
    }
}


fn build_cube() -> Stack<MyBlock> {
    // Often, you will build upon an empty stack.
    let mut stack = Stack::new();

    // Using a closure can simplify block creation and instruction handling.
    let new_block = |id: &str| MyBlock::create(&id.to_string());

    // Stacks can be built from layers, which can be created independently.
    let mut base_layer = Layer::new();
    
    // Adding blocks to a layer will add them to the last row of that layer.
    // If a layer has no rows, it will create one to accommodate the blocks.
    base_layer.add_block(new_block("000"));
    base_layer.add_blocks(vec![
        new_block("001"),
        new_block("002"),
        new_block("003"),
    ]);

    // When you are done building a row, you can start a new one.
    base_layer.new_row();

    // You can also add blocks to the end of a specific row, by index.
    // These will return an error if the row does not exist in the layer.
    base_layer.add_block_to_row(1, new_block("010"))
        .expect("Should add block to newly-created row.");
    base_layer.add_blocks_to_row(1, vec![
        new_block("011"),
        new_block("012"),
        new_block("013"),
    ])
        .expect("Should add many blocks to newly-created row.");

    // Layers can be built from rows, which can be created independently.
    let mut base_row_3 = Row::new();

    // For convenience, most build methods for Row, Layer, and Stack 
    // return a mutable reference to Self, so they can be chained.
    base_row_3
        .add_block(new_block("030"))
        .add_blocks(vec![new_block("031"), new_block("033")])
        .insert_block(2, new_block("032"))
            .expect("Should insert block '032' at index 2.");

    // Adding a row to a layer will place it at the end.
    base_layer.add_row(base_row_3);

    // Row is just a working-wrapper for a vector of blocks.
    let mut row_2 = Row::wrap(vec![
        new_block("020"),
        new_block("021"),
        new_block("022"),
        new_block("023"),
    ]);

    // Rows can be inserted into a specific index in the layer.
    // Returns an error if the layer contains fewer rows then the value given. 

    // DEBUG: THIS FAILS HERE AND IN TESTS
    base_layer.insert_row(2, row_2)
        .expect("Should insert row_2 before the previously-added row.");

    // Once built, add the layer to the top of the stack.
    stack.add_layer(base_layer);


    // Layers can be rapidly populated with generic blocks.
    let mut layer_1 = Layer::new();
    layer_1.populate_with_clones(
        // This macro defines the lengths of the layer's rows.
        layout![4; 4],
        // "Empty" clones can serve as a template.
        &MyBlock::void()
    );

    // Blocks can be modified in place using a partial refrence.
    // Partial refs are organized into vectors that reflect the layer structure.
    layer_1.get_all_mut()
        .expect("Returns Some() if any blocks are present.")
        .into_iter()
        .enumerate()
        .for_each(|(r, row_mut)| {
            row_mut.into_iter()
                .enumerate()
                .for_each(|(b, block_mut)| {
                    block_mut.id = format!("1{}{}", r, b)
                });
        });

    stack.add_layer(layer_1);


    // Stacks can also be populated, adding new layers atop any existing ones.
    stack.populate_with_clones(
        // These layouts describe layers in ascending order.
        // Note: If the macro evaluation becomes confused, try using these brackets:
        vec![ layout!(4; 4), layout!(4; 4) ],
        &MyBlock::void()
    );

    // Stacks can also be partially-(de)referenced.
    stack.get_all_mut()
        .expect("Returns Some() if any blocks are present.")
        .into_iter()
        .enumerate()
        .for_each(|(l, layer_mut)| { if l > 1 {
            layer_mut.into_iter()
                .enumerate()
                .for_each(|(r, row_mut)| {
                    row_mut.into_iter()
                        .enumerate()
                        .for_each(|(b, block_mut)| {
                            block_mut.id = format!("{}{}{}", l, r, b)
                        });
                });
        }});


    // That's a 4x4 cube, the roundabout way.
    stack
}

// It can be done faster.
fn build_cube_quickly() -> Stack<MyBlock> {
    let mut stack = Stack::new();
    stack
        .populate_with_clones(vec!{ layout![4; 4]; 4 }, &MyBlock::void())
        .get_all_mut()
            .expect("There will eventually be better iterator/mapping support.")
            .into_iter()
            .enumerate()
            .for_each(|(l, layer_mut)| { 
                layer_mut.into_iter()
                    .enumerate()
                    .for_each(|(r, row_mut)| {
                        row_mut.into_iter()
                            .enumerate()
                            .for_each(|(b, block_mut)| {
                                block_mut.id = format!("{}{}{}", l, r, b)
                            });
                    });
            });
    stack
}

// More complex shapes can be represented in a voxel-like way.
fn build_pyramid(base_length: usize) -> Stack<MyBlock> {

    // Time to build a pyramid.
    let mut stack = Stack::new();

    // Calculate side lengths for the pyramid shape.
    let mut side_length = base_length as i8;
    let mut side_lengths = Vec::new();
    while side_length > 0 {
        side_lengths.push(side_length as usize);
        side_length -= 2;
    }

    // Map side lengths to square layouts.
    let layouts = side_lengths.into_iter().map(|len| layout![len; len]).collect();

    // Create some "real" blocks for the stack using CreationInstructions instead of clones.
    stack.populate(layouts, &"Stone".to_string());

    // Stacks can be cloned into workable Layers...
    let mut layers = stack.clone_into_layers()
        .into_iter()
        // Add a "border" of void blocks surrounding the blocks of each layer,
        // to centralize existing "Stone" blocks into a pyramid shape.
        .map(|mut layer| {
            let side_length = layer.layout().len();
            let border_width = (base_length - side_length) / 2;
            layer
                .pad_x(border_width)
                .pad_y(border_width)
                .offset_x(border_width)
                .offset_y(border_width);
                // Count on a `border` function soon.
            layer
        })
        .collect();

    // ...but remember to set the Stack to keep changes!
    stack.set_from_layers(layers);

    // You can convert all voids into another block type, 
    // for a more scenic effect.
    stack.fill_voids(&"Air".to_string());

    stack // Fit for a king.
}

// TODO: fn build_sphere() -> Stack { todo!{"Algorithm to calculate sphere slices"}}


fn main() {
    let cube_stack = build_cube();
    let quick_cube = build_cube_quickly();

    cube_stack.get_all_ref()
        .unwrap()
        .into_iter()
        .enumerate()
        .for_each(|(l, layer_ref)| {
            layer_ref.into_iter()
                .enumerate()
                .for_each(|(r, row_ref)| {
                    row_ref.into_iter()
                        .enumerate()
                        .for_each(|(b, block_ref)| {
                            let quick_block_ref = quick_cube.get_block_ref(l, r, b)
                                .expect("Should find the corresponding block.");
                            assert_eq!(block_ref.id, quick_block_ref.id);
                        });
                });
        });

    let pyramid_scene = build_pyramid(7);

    let mut stone_counts: Vec<usize> = 
        pyramid_scene.get_all_ref()
            .unwrap()
            .into_iter()
            .map(|layer| {
                layer.iter()
                    .map(|row| {
                        row.iter()
                            .filter(|block| block.id == "Stone")
                            .count()
                    })
                    .sum()
            })
            .collect();

    assert_eq!(stone_counts, vec![7^2, 5^2, 3^2, 1]);
            

    // TODO Assert and partial refs

}
