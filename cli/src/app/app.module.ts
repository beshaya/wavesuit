import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule, ReactiveFormsModule} from '@angular/forms';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HttpClientModule } from '@angular/common/http';

import {
    MatButtonModule,
    MatSelectModule,
    MatSliderModule,
    MatSlideToggleModule,
 } from '@angular/material';

import { AppComponent } from './app.component';
import { WaveSliderComponent } from './controls/wave-slider/wave-slider.component';


@NgModule({
  imports: [ 
    BrowserModule,
    BrowserAnimationsModule, 
    FormsModule, 
    MatButtonModule,
    MatSelectModule,
    MatSliderModule,
    MatSlideToggleModule,
    ReactiveFormsModule,
    HttpClientModule,
  ],
  declarations: [ AppComponent, WaveSliderComponent ],
  bootstrap:    [ AppComponent ],
  exports: [ 
    MatButtonModule,
    MatSelectModule,
    MatSliderModule,
    MatSlideToggleModule,
  ]
})
export class AppModule { }
