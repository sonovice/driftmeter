/* tslint:disable */
export class DriftMeter {
free(): void;
static  new(arg0: number): DriftMeter;

 process_audio(arg0: Float32Array): void;

 hpcp_ptr(): number;

 fft_window(): number;

 offset(): number;

 reset_offset(): void;

 offset_mean(): number;

 offset_median(): number;

}
