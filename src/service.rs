pub struct PwmService {
    managed_channels: Vec<(u8, u8)>,
}

impl PwmService {
    fn new () -> Self {
        PwmService {
            managed_channels: Vec::new(),
        }
    }
}