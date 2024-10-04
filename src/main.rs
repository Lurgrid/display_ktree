use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Clone)]
enum Code {
    Empty,
    Rod,
}

struct KTree {
    value: char,
    childs: Vec<KTree>,
}

impl KTree {
    const NB_SPACE: usize = 8;
    const ROD: char = '|';
    const CORNER: char = '\\';
    const BRANCH: char = '-';

    fn display_ktree(
        &self,
        mut depth: usize,
        mut code: Vec<Code>,
        mut main: bool,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        let mut cur = self;
        loop {
            if depth > 0 {
                if !main {
                    for i in 0..(depth - 1) {
                        match code[i] {
                            Code::Rod => {
                                write!(f, "{:<width$}", Self::ROD, width = Self::NB_SPACE)?
                            }
                            Code::Empty => write!(f, "{:<width$}", "", width = Self::NB_SPACE)?,
                        }
                    }
                    write!(f, "{}", Self::CORNER)?;
                }
                (0..(Self::NB_SPACE - 1)).try_for_each(|_| write!(f, "{}", Self::BRANCH))?;
            }
            write!(f, "{}", cur.value)?;
            let mut iter = cur.childs.iter();
            let child = iter.next();
            if child.is_none() {
                return writeln!(f, "");
            }
            let mut child = unsafe { child.unwrap_unchecked() };
            let mut ncode = code.clone();
            main = true;
            ncode.push(Code::Rod);
            while let Some(next) = iter.next() {
                child.display_ktree(depth + 1, ncode.clone(), main, f)?;
                child = next;
                main = false;
            }
            code.push(Code::Empty);
            depth += 1;
            cur = child;
        }
    }
}

impl Display for KTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.display_ktree(0, vec![], true, f)
    }
}

fn main() {
    let tree = KTree {
        value: 'A',
        childs: vec![
            KTree {
                value: 'B',
                childs: vec![
                    KTree {
                        value: 'D',
                        childs: vec![
                            KTree {
                                value: 'G',
                                childs: vec![
                                    KTree {
                                        value: 'O',
                                        childs: vec![],
                                    },
                                    KTree {
                                        value: 'P',
                                        childs: vec![],
                                    },
                                ],
                            },
                            KTree {
                                value: 'H',
                                childs: vec![KTree {
                                    value: 'Q',
                                    childs: vec![],
                                }],
                            },
                        ],
                    },
                    KTree {
                        value: 'E',
                        childs: vec![
                            KTree {
                                value: 'I',
                                childs: vec![
                                    KTree {
                                        value: 'R',
                                        childs: vec![],
                                    },
                                    KTree {
                                        value: 'S',
                                        childs: vec![],
                                    },
                                ],
                            },
                            KTree {
                                value: 'J',
                                childs: vec![],
                            },
                        ],
                    },
                ],
            },
            KTree {
                value: 'C',
                childs: vec![KTree {
                    value: 'F',
                    childs: vec![
                        KTree {
                            value: 'K',
                            childs: vec![],
                        },
                        KTree {
                            value: 'L',
                            childs: vec![KTree {
                                value: 'T',
                                childs: vec![],
                            }],
                        },
                    ],
                }],
            },
            KTree {
                value: 'M',
                childs: vec![KTree {
                    value: 'N',
                    childs: vec![
                        KTree {
                            value: 'U',
                            childs: vec![],
                        },
                        KTree {
                            value: 'V',
                            childs: vec![],
                        },
                    ],
                }],
            },
        ],
    };
    println!("{}", tree);
    let tree = KTree {
        value: 'a',
        childs: vec![
            KTree {
                value: 'a',
                childs: vec![
                    KTree {
                        value: 'a',
                        childs: vec![
                            KTree {
                                value: 'a',
                                childs: vec![
                                    KTree {
                                        value: 'a',
                                        childs: vec![],
                                    },
                                    KTree {
                                        value: 'b',
                                        childs: vec![],
                                    },
                                ],
                            },
                            KTree {
                                value: 'b',
                                childs: vec![],
                            },
                        ],
                    },
                    KTree {
                        value: 'b',
                        childs: vec![],
                    },
                ],
            },
            KTree {
                value: 'b',
                childs: vec![],
            },
        ],
    };
    println!("{}", tree);
}
