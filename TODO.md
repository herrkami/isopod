# isopoda

Minimal audio synthesis library for non-FPU architectures

### Todo

- [ ] Set everything to unsigned that doesn't need to be signed.  
- [ ] Check division/shift for performance optimizations.  
- [ ] Unify everything to `i16` signal and `u32` frequency.  
- [ ] Implement overflow handling.
- [ ] Replace if-clause in `Engine::_next()` by masked addition
- [ ] Add `ns` and its doctest.
- [ ] Add linear interpolation iterator to wavetable.
  - [ ] Reduce the size of `exp_*` and add linear interpolation.
- [ ] Add Corsini noise.

### In Progress

### Done ✓
- [x] Finish doc for `Signal` functions
- [x] Introduce signal newtype.
- [x] Add `ExpDecay` to wavetable.rs
- [x] Add doc tests for `us` and `ms`.
- [x] Implement `Period` and `Frequency` for `us` and `ms`.
- [x] Replace the macro in wavetable.rs by a solution that takes a nested
  struct. This might become unnecessary if all signals are set to `i16`.  
- [x] Sign of alpha in wavetable.rs  

