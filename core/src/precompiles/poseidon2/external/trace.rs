use std::borrow::BorrowMut;

use p3_field::PrimeField;
use p3_matrix::dense::RowMajorMatrix;

use crate::{
    air::Word,
    memory::MemoryCols,
    runtime::Segment,
    utils::{ec::NUM_WORDS_FIELD_ELEMENT, Chip},
};

use super::{
    columns::{
        Poseidon2ExternalCols, NUM_POSEIDON2_EXTERNAL_COLS, POSEIDON2_DEFAULT_EXTERNAL_ROUNDS,
    },
    Poseidon2ExternalChip,
};

// I just copied and pasted these from sha compress as a starting point. Carefully examine the code
// and update it. Most computation doesn't make sense for Poseidon2.
impl<F: PrimeField, const N: usize> Chip<F> for Poseidon2ExternalChip<N> {
    fn generate_trace(&self, segment: &mut Segment) -> RowMajorMatrix<F> {
        let mut rows = Vec::new();

        let mut new_field_events = Vec::new();

        for i in 0..segment.poseidon2_external_events.len() {
            let event = segment.poseidon2_external_events[i];
            if i == 0 {
                println!("event: {:#?}", event);
            }
            let mut next_clock: u32 = event.clk;

            //let og_h = event.h;
            //let mut v = [0u32; 8].map(Word::from);

            // let mut octet_num_idx = 0;

            // Load a, b, c, d, e, f, g, h.
            // for j in 0..8usize {
            for round in 0..POSEIDON2_DEFAULT_EXTERNAL_ROUNDS {
                for j in 0..N {
                    let mut row = [F::zero(); NUM_POSEIDON2_EXTERNAL_COLS];
                    let cols: &mut Poseidon2ExternalCols<F> = row.as_mut_slice().borrow_mut();

                    cols.0.segment = F::from_canonical_u32(segment.index);
                    cols.0.clk = F::from_canonical_u32(next_clock);
                    next_clock += 4;
                    cols.0.state_ptr = F::from_canonical_u32(event.state_ptr);
                    //                cols.w_and_h_ptr = F::from_canonical_u32(event.w_and_h_ptr);

                    //                 cols.octet[j] = F::one();
                    //                 cols.octet_num[octet_num_idx] = F::one();
                    //
                    // cols.mem
                    //     .populate_read(event.h_read_records[j], &mut new_field_events);
                    // cols.mem_addr = F::from_canonical_u32(event.w_and_h_ptr + (64 * 4 + j * 4) as u32);
                    cols.0.mem[round]
                        .populate_read(event.state_reads[round][j], &mut new_field_events);
                    cols.0.mem_addr[round] =
                        F::from_canonical_u32(event.state_ptr + (j * 4) as u32);

                    // TODO: Remove this printf-debugging statement.
                    // println!("new_field_events: {:?}", new_field_events);
                    println!(
                        "event.state_reads[{}].value: {:?}",
                        j, event.state_reads[round][j].value,
                    );

                    // cols.a = v[0];
                    // cols.b = v[1];
                    // cols.c = v[2];
                    // cols.d = v[3];
                    // cols.e = v[4];
                    // cols.f = v[5];
                    // cols.g = v[6];
                    // cols.h = v[7];

                    // match j {
                    //     0 => cols.a = *cols.mem.value(),
                    //     1 => cols.b = *cols.mem.value(),
                    //     2 => cols.c = *cols.mem.value(),
                    //     3 => cols.d = *cols.mem.value(),
                    //     4 => cols.e = *cols.mem.value(),
                    //     5 => cols.f = *cols.mem.value(),
                    //     6 => cols.g = *cols.mem.value(),
                    //     7 => cols.h = *cols.mem.value(),
                    //     _ => panic!("unsupported j"),
                    // };

                    // v[0] = cols.a;
                    // v[1] = cols.b;
                    // v[2] = cols.c;
                    // v[3] = cols.d;
                    // v[4] = cols.e;
                    // v[5] = cols.f;
                    // v[6] = cols.g;
                    // v[7] = cols.h;

                    cols.0.is_real = F::one();
                    cols.0.is_external = F::one();
                    // TODO: Remove this before opening a PR.
                    // println!("cols: {:#?}", cols);
                    rows.push(row);
                }
            }

            // Peforms the compress operation.
            // for j in 0..64 {
            //     // if j % 8 == 0 {
            //     //     octet_num_idx += 1;
            //     // }
            //     let mut row = [F::zero(); NUM_POSEIDON2_EXTERNAL_COLS];
            //     let cols: &mut Poseidon2ExternalCols<F> = row.as_mut_slice().borrow_mut();

            //     cols.is_compression = F::one();
            //     cols.octet[j % 8] = F::one();
            //     cols.octet_num[octet_num_idx] = F::one();

            //     cols.segment = F::from_canonical_u32(segment.index);
            //     let clk = event.clk + (8 * 4 + j * 4) as u32; //     cols.clk = F::from_canonical_u32(clk);
            //     cols.w_and_h_ptr = F::from_canonical_u32(event.w_and_h_ptr);
            //     cols.mem
            //         .populate_read(event.w_i_read_records[j], &mut new_field_events);
            //     cols.mem_addr = F::from_canonical_u32(event.w_and_h_ptr + (j * 4) as u32);

            //     let a = event.h[0];
            //     let b = event.h[1];
            //     let c = event.h[2];
            //     let d = event.h[3];
            //     let e = event.h[4];
            //     let f = event.h[5];
            //     let g = event.h[6];
            //     let h = event.h[7];
            //     cols.a = Word::from(a);
            //     cols.b = Word::from(b);
            //     cols.c = Word::from(c);
            //     cols.d = Word::from(d);
            //     cols.e = Word::from(e);
            //     cols.f = Word::from(f);
            //     cols.g = Word::from(g);
            //     cols.h = Word::from(h);

            //     let e_rr_6 = cols.e_rr_6.populate(segment, e, 6);
            //     let e_rr_11 = cols.e_rr_11.populate(segment, e, 11);
            //     let e_rr_25 = cols.e_rr_25.populate(segment, e, 25);
            //     let s1_intermeddiate = cols.s1_intermediate.populate(segment, e_rr_6, e_rr_11);
            //     let s1 = cols.s1.populate(segment, s1_intermeddiate, e_rr_25);

            //     let e_and_f = cols.e_and_f.populate(segment, e, f);
            //     let e_not = cols.e_not.populate(segment, e);
            //     let e_not_and_g = cols.e_not_and_g.populate(segment, e_not, g);
            //     let ch = cols.ch.populate(segment, e_and_f, e_not_and_g);

            //     let temp1 = cols.temp1.populate(h, s1, ch, event.w[j], ch);

            //     let a_rr_2 = cols.a_rr_2.populate(segment, a, 2);
            //     let a_rr_13 = cols.a_rr_13.populate(segment, a, 13);
            //     let a_rr_22 = cols.a_rr_22.populate(segment, a, 22);
            //     let s0_intermediate = cols.s0_intermediate.populate(segment, a_rr_2, a_rr_13);
            //     let s0 = cols.s0.populate(segment, s0_intermediate, a_rr_22);

            //     let a_and_b = cols.a_and_b.populate(segment, a, b);
            //     let a_and_c = cols.a_and_c.populate(segment, a, c);
            //     let b_and_c = cols.b_and_c.populate(segment, b, c);
            //     let maj_intermediate = cols.maj_intermediate.populate(segment, a_and_b, a_and_c);
            //     let maj = cols.maj.populate(segment, maj_intermediate, b_and_c);

            //     let temp2 = cols.temp2.populate(segment, s0, maj);

            //     let d_add_temp1 = cols.d_add_temp1.populate(segment, d, temp1);
            //     let temp1_add_temp2 = cols.temp1_add_temp2.populate(segment, temp1, temp2);

            //     event.h[7] = g;
            //     event.h[6] = f;
            //     event.h[5] = e;
            //     event.h[4] = d_add_temp1;
            //     event.h[3] = c;
            //     event.h[2] = b;
            //     event.h[1] = a;
            //     event.h[0] = temp1_add_temp2;

            //     cols.is_real = F::one();

            //     rows.push(row);
            // }

            // let mut v: [u32; 8] = (0..8)
            //     .map(|i| event.h[i])
            //     .collect::<Vec<_>>()
            //     .try_into()
            //     .unwrap();

            // octet_num_idx += 1;
            // // Store a, b, c, d, e, f, g, h.
            // for j in 0..8usize {
            for round in 0..POSEIDON2_DEFAULT_EXTERNAL_ROUNDS {
                for j in 0..NUM_WORDS_FIELD_ELEMENT {
                    let mut row = [F::zero(); NUM_POSEIDON2_EXTERNAL_COLS];
                    let cols: &mut Poseidon2ExternalCols<F> = row.as_mut_slice().borrow_mut();

                    cols.0.segment = F::from_canonical_u32(segment.index);
                    // let clk = event.clk + (NUM_WORDS_FIELD_ELEMENT * 4 + (j * 4)) as u32;
                    cols.0.clk = F::from_canonical_u32(next_clock);
                    next_clock += 4;
                    cols.0.state_ptr = F::from_canonical_u32(event.state_ptr);
                    // cols.w_and_h_ptr = F::from_canonical_u32(event.w_and_h_ptr);

                    // cols.octet[j] = F::one();
                    // cols.octet_num[octet_num_idx] = F::one();

                    // cols.finalize_add.populate(segment, og_h[j], event.h[j]);
                    // cols.mem
                    //     .populate_write(event.h_write_records[j], &mut new_field_events);
                    // cols.mem_addr = F::from_canonical_u32(event.w_and_h_ptr + (64 * 4 + j * 4) as u32);
                    cols.0.mem[round]
                        .populate_write(event.state_writes[round][j], &mut new_field_events);
                    cols.0.mem_addr[round] =
                        F::from_canonical_u32(event.state_ptr + (j * 4) as u32);

                    // v[j] = event.h[j];
                    // cols.a = Word::from(v[0]);
                    // cols.b = Word::from(v[1]);
                    // cols.c = Word::from(v[2]);
                    // cols.d = Word::from(v[3]);
                    // cols.e = Word::from(v[4]);
                    // cols.f = Word::from(v[5]);
                    // cols.g = Word::from(v[6]);
                    // cols.h = Word::from(v[7]);

                    // match j {
                    //     0 => cols.finalized_operand = cols.a,
                    //     1 => cols.finalized_operand = cols.b,
                    //     2 => cols.finalized_operand = cols.c,
                    //     3 => cols.finalized_operand = cols.d,
                    //     4 => cols.finalized_operand = cols.e,
                    //     5 => cols.finalized_operand = cols.f,
                    //     6 => cols.finalized_operand = cols.g,
                    //     7 => cols.finalized_operand = cols.h,
                    //     _ => panic!("unsupported j"),
                    // };

                    cols.0.is_real = F::one();
                    cols.0.is_external = F::one();

                    rows.push(row);
                }
            }
        }

        segment.field_events.extend(new_field_events);

        let nb_rows = rows.len();
        let mut padded_nb_rows = nb_rows.next_power_of_two();
        if padded_nb_rows == 2 || padded_nb_rows == 1 {
            padded_nb_rows = 4;
        }

        for _ in nb_rows..padded_nb_rows {
            let row = [F::zero(); NUM_POSEIDON2_EXTERNAL_COLS];
            rows.push(row);
        }

        // Convert the trace to a row major matrix.
        RowMajorMatrix::new(
            rows.into_iter().flatten().collect::<Vec<_>>(),
            NUM_POSEIDON2_EXTERNAL_COLS,
        )
    }

    fn name(&self) -> String {
        "Poseidon2External".to_string()
    }
}