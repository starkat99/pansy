#![feature(allocator_api)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

extern crate alloc;

use alloc::alloc::{Allocator, Global};
use core::{any::Any, cell::Cell, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct VirtualMachineConfig<A: Allocator = Global> {
    allocator: A,
}

impl VirtualMachineConfig<Global> {
    #[inline]
    pub fn with_global_allocator() -> Self {
        Self::with_allocator(Global)
    }
}

impl<A: Allocator> VirtualMachineConfig<A> {
    pub fn with_allocator(allocator: A) -> Self {
        VirtualMachineConfig { allocator }
    }

    pub fn build(self) -> VirtualMachine<A> {
        VirtualMachine {
            allocator: self.allocator,
            heap_size: Cell::new(0),
        }
    }
}

impl<A: Allocator + Default> Default for VirtualMachineConfig<A> {
    #[inline]
    fn default() -> Self {
        Self::with_allocator(A::default())
    }
}

#[derive(Debug)]
pub struct VirtualMachine<A: Allocator = Global> {
    allocator: A,
    heap_size: Cell<usize>,
}

impl<A: Allocator> VirtualMachine<A> {
    #[inline]
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    #[inline]
    pub fn heap_size(&self) -> usize {
        self.heap_size.get()
    }
}

pub type Integer = i64;
pub type Float = f64;

#[derive(Debug, Clone, Copy, Default)]
pub enum Value {
    #[default]
    None,
    Bool(bool),
    Integer(Integer),
    Float(Float),
    Reference(NonNull<HeapObject>),
}

#[derive(Debug)]
pub struct HeapObject {
    obj_type: ObjectType,
    obj: dyn Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    List,
    String,
    Table,
    Function,
}

pub trait Object: core::fmt::Debug + Any {}
