/// garmin connect has a workout training feature, which allows you define a workout, upload it to the watch, and do it.
/// Workouts are made of steps, for instance warmup, run, recover, loop, with their numbering... ,  
///
/// after a workout, you can export it to gpx, tcx or kml format, each has its pros and cons
/// cons :
/// - gpx : everyhing is in one step
/// - tcx : fine, but cannot be loaded in google earth
/// - kml does not have the speed, heart rate... for each point
/// - we always miss the step name
/// - we want to have custom colors and markers
///
/// garmin connect will also export a csv file, that contains the workout summary. It is interesting because it has the step name.
///
/// so the tool will :
/// - load a garming tcx file to a rust polars dataframe, the lap list will be exploded, with a circuit field ( starting at index 1 )
/// - optionaly merge with the csv, to get the workout interval name
/// - apply a coloring function
/// - save to kml, to be displayed in googe earth
pub mod model;

/// read garmin tcx file and returns a polars dataframe
pub mod read;
