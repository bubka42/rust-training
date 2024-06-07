use p42::song;

fn main() -> std::io::Result<()> {
    song::song_to_tcp(song::SongIter::default(), "127.0.0.1:3000")
}
