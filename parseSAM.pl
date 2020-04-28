#!/usr/bin/perl
use 5.010;
use strict;
use warnings;
use feature qw(say);

#this parses the output SAM file (after all pieces have been concatenated and it has been sorted)

my $id = "none";
my $type;
my $side; 
my $name;
my $flag = 0;
my $ratio = 0;
my $infile;
my $outfile;
my $arfCount = 0;
my $srfCount = 0;
my $count = 0;

if(@ARGV == 2){
  $infile = shift;
  $outfile = shift;  
}
else{
  print "Usage: parseSAM.pl [infile] [outfile]\nPlease provide filenames now.\n";
  ($infile, $outfile) = split(" ", <STDIN>);
}

open(FH, '>', $outfile) or die $!;
#print FH "ID\tARFs\tSRFs\t\%ARFs\n"; #original use print statement
print FH "DCE_ID\tHits\n"; #adapted to match output of count.py

open(my $fh, '<:encoding(UTF-8)', $infile)
  or die "Could not open file '$infile' $!";


while (my $line = <$fh>){
	chomp $line; 
	#if($line =~ m/^(\d+)-([AS])-(\S)\s+(\d+).+$/){ #old version
	if($line =~ m/^(\d+-[AS]-\S)\s+(\d+).+$/){  #new version - just grabbing whole ID, not comparing arf vs srf
		if($id eq "none"){
			#first line - set everything up
			$id = $1;
			#$id = $1;
			#$type = $2;
			#$side = $3;
			#$name = "$id-$type-$side";
			#say "name: $name";
			#if($type eq "A"){
				#say "this is an ARF";
			#	$arfCount++;
			#}
			#else{
			#	#say "this is a SRF";
			#	$srfCount++;
			#}
		}

		elsif($1 eq $id){
			#still on the same ID - keep increasing counts
			$count++;
			#$type = $2;
			#$side = $3;
			#if($type eq "A"){
			#	#say "this is an ARF";
			#	$arfCount++;
			#}
			#else{
			#	#say "this is a SRF";
			#	$srfCount++;
			#}
		}

		else{
			#new ID has started - write previous stuff to file
			#my $percent = $arfCount/($arfCount+$srfCount);
			#$ratio = sprintf("%d%%",$percent*100);
			#print FH "$id\t$arfCount\t$srfCount\t$ratio\n";
			print FH "<$id\t$count\n";
			#reset counts/carried over variables and start again
			$count = 0;
			#$arfCount = 0;
			#$srfCount = 0;
			$id = $1;
			#$type = $2;
			#$side = $3;
			#$name = "$id-$type-$side";
			#say "name: $name";
			$count++;
			#if($type eq "A"){
			#	#say "this is an ARF";
			#	$arfCount++;
			#}
			#else{
			#	#say "this is a SRF";
			#	$srfCount++;
			#}

		}
	}
    else{
    	say "Didn't match regex: $line";
    }
}
#print last one to file
#my $percent = $arfCount/($arfCount+$srfCount);
#$ratio = sprintf("%d%%",$percent*100);
print FH "<$id\t$count\n";
#print FH "$id\t$arfCount\t$srfCount\t$ratio\n";
close(FH);