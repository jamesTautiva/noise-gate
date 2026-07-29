use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use noise_gate::dsp::NoiseGateProcessor;
use noise_gate::params::NoiseGateParams;
use std::sync::Arc;

fn benchmark_filter_operations(c: &mut Criterion) {
    let params = Arc::new(NoiseGateParams::default());
    let mut processor = NoiseGateProcessor::new(params.clone());
    
    let mut group = c.benchmark_group("filter_operations");
    
    for num_samples in [64, 128, 256, 512, 1024].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_samples),
            num_samples,
            |b, &num_samples| {
                let mut test_signal: Vec<f32> = (0..num_samples).map(|i| (i as f32 * 0.01).sin()).collect();
                b.iter(|| {
                    // Simulate filter operations
                    let hp_alpha = 0.99;
                    let lp_alpha = 0.01;
                    let mut prev_input = 0.0f32;
                    let mut prev_output = 0.0f32;
                    
                    for sample in test_signal.iter_mut() {
                        let output = hp_alpha * (*sample - prev_input + prev_output);
                        prev_input = *sample;
                        prev_output = output;
                        *sample = output.abs();
                    }
                    
                    black_box(test_signal.clone())
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_envelope_follower(c: &mut Criterion) {
    let mut group = c.benchmark_group("envelope_follower");
    
    for num_samples in [64, 128, 256, 512, 1024].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_samples),
            num_samples,
            |b, &num_samples| {
                let mut test_signal: Vec<f32> = (0..num_samples).map(|i| (i as f32 * 0.01).sin()).collect();
                let attack_coeff = 0.001;
                let release_coeff = 0.01;
                let mut envelope = 0.0f32;
                
                b.iter(|| {
                    for &target in test_signal.iter() {
                        let coeff = if target > envelope {
                            attack_coeff
                        } else {
                            release_coeff
                        };
                        envelope = target + coeff * (envelope - target);
                    }
                    
                    black_box(envelope)
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_gain_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("gain_calculation");
    
    for num_samples in [64, 128, 256, 512, 1024].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_samples),
            num_samples,
            |b, &num_samples| {
                let mut test_signal: Vec<f32> = (0..num_samples).map(|i| (i as f32 * 0.01).sin()).collect();
                let mut envelope = 0.5f32;
                let range_db = -20.0;
                let range_gain = 10.0_f32.powf(range_db / 20.0);
                
                b.iter(|| {
                    for sample in test_signal.iter_mut() {
                        let gate_open = envelope > 0.5;
                        let gain = if gate_open { 1.0 } else { range_gain };
                        *sample = *sample * gain;
                    }
                    
                    black_box(test_signal.clone())
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_filter_operations,
    benchmark_envelope_follower,
    benchmark_gain_calculation
);
criterion_main!(benches);
