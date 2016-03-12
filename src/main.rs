extern crate portaudio;

use std::thread::sleep_ms;

fn demo() -> portaudio::PaResult
{
    let mut stream = Some(portaudio::stream::Stream::open_default(
                          0, // input channels
                          2, // output channels
                          44100.0, // sample rate
                          portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                          None // no callback
                     ).unwrap());

    stream.as_mut().unwrap().start().unwrap();

    let mut phase = 0.0f32;
    let mut buffer = Vec::with_capacity(44100*2);
    for _i in 0..44100*2
    {
        // Small amplitude such that the test does not produce sound
        buffer.push(phase * 0.1);

        phase += 0.01;
        if phase > 1.0 { phase -= 2.0; }
    }

    stream.as_mut().unwrap().write(&buffer).unwrap();
    stream.as_mut().unwrap().stop().unwrap(); //not changing anything
//    drop(stream); //this seems to be called when stream goes out of scope
    stream = None;

    sleep_ms(10000); //only works if called drop before
    Ok(())
}

fn main()
{
    portaudio::initialize().unwrap();
    println!("{:?}", demo());
//    sleep_ms(10000); //works
    println!("terminate");
    portaudio::terminate().unwrap();
}

