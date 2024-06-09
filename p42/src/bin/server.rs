use p42::song;

fn main() -> std::io::Result<()> {
    song::song_from_tcp(3000)
}
