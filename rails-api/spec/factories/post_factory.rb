FactoryGirl.define do
  factory :post do
    title { Faker::Lorem.words(4) }
    body { Faker::Lorem.paragraph(10) }
    published true
  end
end
