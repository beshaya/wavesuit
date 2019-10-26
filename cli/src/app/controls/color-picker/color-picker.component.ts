import { Component, OnInit, Input } from '@angular/core';
import { FormGroup } from '@angular/forms';

import { ColorPickerService, Cmyk } from 'ngx-color-picker';

interface Color {
  r: number,
  g: number,
  b: number,
}

function structToColor(c: Color): string {
  return 'rgb(' + c.r + ',' + c.g + ',' + c.b + ')';
}

@Component({
  selector: 'wave-color-picker',
  templateUrl: './color-picker.component.html',
  styleUrls: ['./color-picker.component.css']
})
export class ColorPickerComponent implements OnInit {
  @Input() form: FormGroup;
  color: string;

  constructor(
    private colorPickerService: ColorPickerService,
  ) { }

  ngOnInit() {
    const val: Color = this.form.value as Color;
    this.color = structToColor(val);
  }

  colorChanged(click: string) {
    const hsva = this.colorPickerService.stringToHsva(click);
    if (hsva) {
      const rgba = this.colorPickerService.hsvaToRgba(hsva);
      const newColor: Color = {
        r: Math.floor(rgba.r * 255),
        g: Math.floor(rgba.g * 255),
        b: Math.floor(rgba.b * 255),
      };
      this.form.setValue(newColor);
      this.color = structToColor(newColor);
    }
  }

}
