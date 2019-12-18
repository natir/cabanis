/*
Copyright (c) 2020 Pierre Marijon <pmarijon@mpi-inf.mpg.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* crate use */
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.1",
    author = "Pierre Marijon <pmarijon@mpi-inf.mpg.de>",
    name = "cabanis",
    about = "Use solid kmer and sequence to build a compacted ABruijn graph in gfa format."
)]
pub struct Command {
    #[structopt(
        short = "s",
        long = "solidity",
        required = true,
        help = "path of solidity input file"
    )]
    pub solidity: String,

    #[structopt(
        short = "g",
        long = "gfa",
        required = true,
        help = "path of gfa output file"
    )]
    pub gfa: String,

    #[structopt(
        short = "f",
        long = "fasta",
        required = true,
        help = "path of fasta output file"
    )]
    pub fasta: String,

    #[structopt(
        short = "k",
        long = "kmer",
        required = true,
        help = "path of kmer graph output file"
    )]
    pub kmer: String,

    #[structopt(
        short = "t",
        long = "edge-weight-threshold",
        default_value = "5",
        help = "All edge in graph are lower than this value"
    )]
    pub edge_threshold: u8,
}