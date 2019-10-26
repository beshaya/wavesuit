import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule, ReactiveFormsModule} from '@angular/forms';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HttpClientModule } from '@angular/common/http';
import { ColorPickerModule } from 'ngx-color-picker';
import { NgSelectModule } from '@ng-select/ng-select';

import {
    MatButtonModule,
    MatSelectModule,
    MatSliderModule,
    MatSlideToggleModule,
 } from '@angular/material';

import { AppComponent } from './app.component';
import { WaveSliderComponent } from './controls/wave-slider/wave-slider.component';
import { ColorPickerComponent } from './controls/color-picker/color-picker.component';


@NgModule({
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    FormsModule,
    MatButtonModule,
    ColorPickerModule,
    MatSelectModule,
    MatSliderModule,
    MatSlideToggleModule,
    NgSelectModule,
    ReactiveFormsModule,
    HttpClientModule,
  ],
  declarations: [ AppComponent, WaveSliderComponent, ColorPickerComponent ],
  bootstrap:    [ AppComponent ],
  exports: [
    MatButtonModule,
    MatSelectModule,
    MatSliderModule,
    MatSlideToggleModule,
  ]
})
export class AppModule { }
