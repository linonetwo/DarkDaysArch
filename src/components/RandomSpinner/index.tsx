import React from 'react';
import { sample } from 'lodash';
import styled from 'styled-components';

import {
  AtomSpinner,
  BreedingRhombusSpinner,
  CirclesToRhombusesSpinner,
  FingerprintSpinner,
  FlowerSpinner,
  FulfillingBouncingCircleSpinner,
  FulfillingSquareSpinner,
  HalfCircleSpinner,
  HollowDotsSpinner,
  IntersectingCirclesSpinner,
  LoopingRhombusesSpinner,
  OrbitSpinner,
  PixelSpinner,
  RadarSpinner,
  ScalingSquaresSpinner,
  SelfBuildingSquareSpinner,
  SemipolarSpinner,
  SpringSpinner,
  SwappingSquaresSpinner,
  TrinityRingsSpinner,
} from 'react-epic-spinners';
import {
  PulseLoader,
  RotateLoader,
  BeatLoader,
  RiseLoader,
  SyncLoader,
  GridLoader,
  ClipLoader,
  FadeLoader,
  ScaleLoader,
  SquareLoader,
  PacmanLoader,
  SkewLoader,
  RingLoader,
  MoonLoader,
  DotLoader,
  BounceLoader,
} from 'halogenium';

const epicSpinners = [
  AtomSpinner,
  BreedingRhombusSpinner,
  CirclesToRhombusesSpinner,
  FingerprintSpinner,
  FlowerSpinner,
  FulfillingBouncingCircleSpinner,
  FulfillingSquareSpinner,
  HalfCircleSpinner,
  HollowDotsSpinner,
  IntersectingCirclesSpinner,
  LoopingRhombusesSpinner,
  OrbitSpinner,
  PixelSpinner,
  RadarSpinner,
  ScalingSquaresSpinner,
  SelfBuildingSquareSpinner,
  SemipolarSpinner,
  SpringSpinner,
  SwappingSquaresSpinner,
  TrinityRingsSpinner,
];
const halogenSpinners = [
  PulseLoader,
  RotateLoader,
  BeatLoader,
  RiseLoader,
  SyncLoader,
  GridLoader,
  ClipLoader,
  FadeLoader,
  ScaleLoader,
  SquareLoader,
  PacmanLoader,
  SkewLoader,
  RingLoader,
  MoonLoader,
  DotLoader,
  BounceLoader,
];

const RandomSpinnerContainer = styled.div``;

export function RandomSpinner(props: { loading?: boolean; size?: number }): JSX.Element {
  if (!(props.loading ?? false)) return <span></span>;
  const SpinnerElement = sample([...epicSpinners, ...halogenSpinners]);
  if (SpinnerElement !== undefined) {
    return (
      <RandomSpinnerContainer>
        <SpinnerElement color="green" className="loading-spinner" size={props.size ?? 24} />
      </RandomSpinnerContainer>
    );
  }
  return <RandomSpinnerContainer>loading...</RandomSpinnerContainer>;
}
