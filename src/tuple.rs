pub struct Tuple<A, B>(A, B);

impl<A, B> Tuple<A, B>
where
    A: Clone,
    B: Clone,
{
    pub fn as_rust(&self) -> (A, B) {
        (self.0.clone(), self.1.clone())
    }

    pub fn first(&self) -> A {
        self.0.clone()
    }

    pub fn second(&self) -> B {
        self.1.clone()
    }

    pub fn of(a: A, b: B) -> Self {
        Self(a, b)
    }

    pub fn from_rust(v: (A, B)) -> Self {
        Self(v.0, v.1)
    }
}

impl Tuple<u32, u32> {
    pub fn as_slice(&self) -> [u32; 2] {
        [self.0, self.1]
    }
}

impl<A, B> Clone for Tuple<A, B>
where
    A: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<A, B> Copy for Tuple<A, B>
where
    A: Copy,
    B: Copy,
{
}

pub struct Triple<A, B, C>(A, B, C);

impl<A, B, C> Triple<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    pub fn as_rust(&self) -> (A, B, C) {
        (self.0.clone(), self.1.clone(), self.2.clone())
    }

    pub fn first(&self) -> A {
        self.0.clone()
    }

    pub fn second(&self) -> B {
        self.1.clone()
    }

    pub fn third(&self) -> C {
        self.2.clone()
    }

    pub fn of(a: A, b: B, c: C) -> Self {
        Self(a, b, c)
    }

    pub fn from_rust(v: (A, B, C)) -> Self {
        Self(v.0, v.1, v.2)
    }
}

impl Triple<u32, u32, u32> {
    pub fn as_slice(&self) -> [u32; 3] {
        [self.0, self.1, self.2]
    }
}

impl<A, B, C> Clone for Triple<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<A, B, C> Copy for Triple<A, B, C>
where
    A: Copy,
    B: Copy,
    C: Copy,
{
}
