# isopoda

Minimal audio synthesis library for non-FPU architectures

### Todo

- [ ] Set everything to unsigned that doesn't need to be signed.  
- [ ] Check division/shift for performance optimizations.  
- [ ] Unify everything to `i16` signal and `u32` frequency.  
- [ ] Implement overflow handling.
- [ ] Replace if-clause in `Engine::_next()` by masked addition

### In Progress

- [ ] Add `ns`.
- [ ] Add doc tests for `us` and `ms`.

### Done ✓

- [x] Implement `Period` and `Frequency` for `us` and `ms`.
- [x] Replace the macro in wavetable.rs by a solution that takes a nested
  struct. This might become unnecessary if all signals are set to `i16`.  
- [x] Sign of alpha in wavetable.rs  

