pub struct Tuple<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Tuple<A, B>
where
    A: Clone,
    B: Clone,
{
    pub fn as_rust(&self) -> (A, B) {
        (self.a.clone(), self.b.clone())
    }

    pub fn first(&self) -> A {
        self.a.clone()
    }

    pub fn second(&self) -> B {
        self.b.clone()
    }

    pub fn of(a: A, b: B) -> Self {
        Self { a, b }
    }

    pub fn from_rust(v: (A, B)) -> Self {
        Self { a: v.0, b: v.1 }
    }
}

impl Tuple<u32, u32> {
    pub fn as_slice(&self) -> [u32; 2] {
        [self.a, self.b]
    }
}

impl<A, B> Clone for Tuple<A, B>
where
    A: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl<A, B> Copy for Tuple<A, B>
where
    A: Copy,
    B: Copy,
{
}

pub struct Triple<A, B, C> {
    pub a: A,
    pub b: B,
    pub c: C,
}

impl<A, B, C> Triple<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    pub fn as_rust(&self) -> (A, B, C) {
        (self.a.clone(), self.b.clone(), self.c.clone())
    }

    pub fn first(&self) -> A {
        self.a.clone()
    }

    pub fn second(&self) -> B {
        self.b.clone()
    }

    pub fn third(&self) -> C {
        self.c.clone()
    }

    pub fn of(a: A, b: B, c: C) -> Self {
        Self { a, b, c }
    }

    pub fn from_rust(v: (A, B, C)) -> Self {
        Self {
            a: v.0,
            b: v.1,
            c: v.2,
        }
    }
}

impl Triple<u32, u32, u32> {
    pub fn as_slice(&self) -> [u32; 3] {
        [self.a, self.b, self.c]
    }
}

impl<A, B, C> Clone for Triple<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
        }
    }
}

impl<A, B, C> Copy for Triple<A, B, C>
where
    A: Copy,
    B: Copy,
    C: Copy,
{
}
