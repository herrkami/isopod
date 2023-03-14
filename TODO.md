# isopoda

Minimal audio synthesis library for non-FPU architectures

### Todo

- [ ] Set everything to unsigned that doesn't need to be signed.  
  - [ ] Especially alpha in wavetable.rs  
- [ ] Check division/shift for performance optimizations.  
- [ ] Unify everything to `i16` signal and `u32` frequency.  
- [ ] Implement overflow handling.  

### In Progress


### Done ✓

- [x] Replace the macro in wavetable.rs by a solution that takes a nested
  struct. This might become unnecessary if all signals are set to `i16`.  

