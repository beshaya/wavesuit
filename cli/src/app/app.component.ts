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
  painterParams: PainterParams;

  private server: string = "/api";

  painters = [
    {value: "hex", viewValue: "Hex"},
    {value: "line", viewValue: "Line"},
    {value: "pulse", viewValue: "Pulse"},
    {value: "rain", viewValue: "Rain"},
    {value: "fade", viewValue: "Fade"},
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
      this.painterParams = data;
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

  save() {
    console.log("Save");
    console.log(this.form.value);
    this.http.post(this.server, this.form.value, httpOptions)
      .subscribe((response) => console.log(response));
  }

  load() {
    this.http.get(this.server).subscribe((data: PainterParams) => {
      this.painterParams = data;

    });
  }
}
