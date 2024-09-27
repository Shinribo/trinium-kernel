pub(crate) mod irqlvmutex;
pub(crate) mod irqlvrwlock;


enum WaitStrategy {
    Spin,
    Yield,
}