use std::marker::PhantomData;
use super::address::Address;






pub enum ClientError {}


pub struct Client<T> 
{
    addresses: Vec<Address>, 
    state: PhantomData<T>
}

impl<T> Client<T>
{
    pub fn new(addresses: Vec<Address>) -> Client<UnInitialisedMode> {
        Client {addresses, state: PhantomData::<UnInitialisedMode> }
    }
}

impl<T> Client<T>
where 
    T: UnInitialisedState
{
    pub async fn connect(self) -> Result<Client<LockingMode>, (Client<UnInitialisedMode>, ClientError)> { 
        unimplemented!();
    }
}

impl<T> Client<T>
where
    T: LockingState
{
    pub async fn lock(self) -> Result<Client<UnlockingMode>, ()> {
        unimplemented!();
    }
}

impl<T> Client<T>
where
    T: UnlockingState
{
    pub async fn unlock(self) -> Result<Client<LockingMode>, ()> {
        unimplemented!();
    }
}

pub trait LockingState {}
pub trait UnlockingState {}
pub trait UnInitialisedState {}

#[derive(Debug)]
pub enum LockingMode {}

#[derive(Debug)]
pub enum UnlockingMode {}


#[derive(Debug)]
pub enum UnInitialisedMode {}

impl LockingState for LockingMode {}
impl UnlockingState for UnlockingMode {}

