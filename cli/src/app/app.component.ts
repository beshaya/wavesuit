import { Component } from '@angular/core';
import { FormBuilder, FormGroup, FormArray } from '@angular/forms';
import { HttpClient, HttpHeaders } from '@angular/common/http';

interface Color {
  r: number,
  g: number,
  b: number,
}

interface PainterParams {
  painter: string,
  global_brightness: number,
  speed: number,
  color: Color,
  secondary_colors: Color[],
  fade: number,
  bidirectional: boolean,
}

const httpOptions = {
  headers: new HttpHeaders({
    'Content-Type': 'application/json',
  })
};

@Component({
  selector: 'my-app',
  templateUrl: './app.component.html',
  styleUrls: [ './app.component.css' ]
})
export class AppComponent  {
  form: FormGroup;
  private initialParams: PainterParams;
  private server: string = "/api";

  painters = [
    {id: 'hex', text: "Hex"},
    {id: "line", text: "Line"},
    {id: "pulse", text: "Pulse"},
    {id: "rain", text: "Rain"},
    {id: "fade", text: "Fade"},
    {id: "disco", text: "Disco"},
  ];

  constructor(
    private formBuilder: FormBuilder,
    private http: HttpClient) {
  }

  ngOnInit() {
    this.http.get(this.server).subscribe((data: PainterParams) => {
      this.form = this.formBuilder.group({
        painter: [data.painter],
        global_brightness: [data.global_brightness],
        speed: [data.speed],
        color: this.colorGroup(data.color),
        secondary_colors: this.formBuilder.array(
          data.secondary_colors.map((c: Color) => this.colorGroup(c))
        ),
        fade: [data.fade],
        bidirectional: [data.bidirectional]
      });
      this.initialParams = data;
      this.form.valueChanges.subscribe((val: PainterParams) => {
        this.write(val);
      });
    });
  }

  private colorGroup(color?: Color): FormGroup {
    if (!color) {
      color = {r: 0, g: 0, b: 0};
    }
    return this.formBuilder.group({
      r: [color.r],
      g: [color.g],
      b: [color.b],
    })
  }

  addColor() {
    (this.form.controls['secondary_colors'] as FormArray).push(this.colorGroup());
  }

  removeColor(index: number) {
    (this.form.controls['secondary_colors'] as FormArray).removeAt(index);
  }

  private write(val: PainterParams) {
    this.http.post(this.server, this.form.value, httpOptions)
      .subscribe((response) => console.log(response));
  }

  save() {
  }

  load() {
    this.form.setValue(this.initialParams);
  }
}
